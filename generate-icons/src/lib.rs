use roxmltree::{Document, Node};
use std::{
    fmt::{self, Display, Write},
    fs,
    path::PathBuf,
};

pub trait Icon {
    fn path(&self) -> PathBuf;
    fn const_name(&self) -> String;
    fn size(&self) -> kurbo::Size;

    fn shapes(&self) -> Vec<KurboShape> {
        println!("Getting {}", self.path().display());
        let raw = fs::read_to_string(self.path()).unwrap();
        let doc = Document::parse(&raw).unwrap();
        let svg = doc.root_element();
        assert!(svg.has_tag_name("svg"));
        assert_eq!(
            svg.attribute("width").unwrap(),
            IconSize(self.size()).to_string()
        );
        assert_eq!(
            svg.attribute("height").unwrap(),
            IconSize(self.size()).to_string()
        );
        svg.children().map(|el| KurboShape::from_svg(el)).collect()
    }
}

pub struct IconSize(kurbo::Size);

impl IconSize {
    pub fn new(size: kurbo::Size) -> Self {
        IconSize(size)
    }
}

impl Display for IconSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.width == self.0.height {
            write!(f, "{}", self.0.width)
        } else {
            write!(f, "{}x{}", self.0.width, self.0.height)
        }
    }
}

#[derive(Debug)]
pub enum KurboShape {
    Circle(kurbo::Circle),
    BezPath(kurbo::BezPath),
}

impl KurboShape {
    pub fn from_svg(input: Node) -> Self {
        match input.tag_name().name() {
            "circle" => {
                let cx = input.attribute("cx").unwrap().parse::<f64>().unwrap();
                let cy = input.attribute("cy").unwrap().parse::<f64>().unwrap();
                let r = input.attribute("r").unwrap().parse::<f64>().unwrap();
                KurboShape::Circle(kurbo::Circle::new((cx, cy), r))
            }
            "path" => {
                let d = input.attribute("d").unwrap();
                KurboShape::BezPath(kurbo::BezPath::from_svg(d).unwrap())
            }
            other => panic!("unrecognised node: {}", other),
        }
    }
}

impl Display for KurboShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KurboShape::Circle(circle) => write!(
                f,
                "IconShape::Circle(Circle {{ center: {}, radius: {:.2} }})",
                KurboPoint(circle.center),
                circle.radius
            ),
            KurboShape::BezPath(path) => {
                f.write_str("IconShape::PathEls(&[")?;
                for el in path.iter() {
                    write!(f, "\n            {},", KurboEl(el))?;
                }
                f.write_str("\n        ])")
            }
        }
    }
}

pub struct KurboPoint(kurbo::Point);

impl Display for KurboPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ x: {:.2}, y: {:.2} }}", self.0.x, self.0.y)
    }
}

pub struct KurboSize(kurbo::Size);

impl Display for KurboSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Size {{ width: {:.2}, height: {:.2} }}",
            self.0.width, self.0.height
        )
    }
}

pub struct KurboEl(kurbo::PathEl);

impl Display for KurboEl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use kurbo::PathEl;
        match self.0 {
            PathEl::MoveTo(point) => write!(f, "PathEl::MoveTo({})", KurboPoint(point)),
            PathEl::LineTo(point) => write!(f, "PathEl::LineTo({})", KurboPoint(point)),
            PathEl::QuadTo(point1, point2) => write!(
                f,
                "PathEl::QuadTo({}, {})",
                KurboPoint(point1),
                KurboPoint(point2)
            ),
            PathEl::CurveTo(point1, point2, point3) => write!(
                f,
                "PathEl::CurveTo({}, {}, {})",
                KurboPoint(point1),
                KurboPoint(point2),
                KurboPoint(point3)
            ),
            PathEl::ClosePath => f.write_str("PathEl::ClosePath"),
        }
    }
}

pub struct Implement<'a, I: Icon>(&'a I);

impl<'a, I: Icon> Implement<'a, I> {
    pub fn new(icon: &'a I) -> Self {
        Implement(icon)
    }
}

impl<I: Icon> Display for Implement<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut shapes = String::new();
        for shape in self.0.shapes() {
            writeln!(shapes, "\n        {},", shape)?;
        }
        write!(
            f,
            r#"
pub const {}: IconShapes = IconShapes {{
    shapes: &[{}    ],
    size: {},
}};
        "#,
            self.0.const_name(),
            shapes,
            KurboSize(self.0.size())
        )
    }
}
