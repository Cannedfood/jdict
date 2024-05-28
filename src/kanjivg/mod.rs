pub mod path;
use roxmltree::NodeType;

pub use self::path::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumString, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
pub enum Radical {
    #[strum(serialize = "tradit")]  Traditional,
    #[strum(serialize = "general")] General,
    #[strum(serialize = "nelson")]  Nelson,
    #[strum(serialize = "jis")]     Jis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumString, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
pub enum Position {
    #[strum(serialize = "top")]    Top,
    #[strum(serialize = "bottom")] Bottom,
    #[strum(serialize = "left")]   Left,
    #[strum(serialize = "right")]  Right,
    #[strum(serialize = "tare")]   Tare,
    #[strum(serialize = "tarec")]  TareCenter,
    #[strum(serialize = "kamae")]  Kamae,
    #[strum(serialize = "nyo")]    Nyo,
    #[strum(serialize = "nyoc")]   NyoCenter,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct StrokeGroup {
    pub variant: bool,
    pub radical: Option<Radical>,
    pub original: Option<char>,
    pub position: Option<Position>,
    pub part: Option<u8>,
    pub number: Option<u8>,
    pub partial: bool,
    pub phon: Option<char>,
    pub radical_form: bool,
    pub trad_form: bool,

    pub element:   Option<char>,
    pub subgroups: Vec<Child>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Child {
    Stroke(Stroke),
    Group(StrokeGroup),
}
impl From<Stroke> for Child {
    fn from(stroke: Stroke) -> Self { Child::Stroke(stroke) }
}
impl From<StrokeGroup> for Child {
    fn from(group: StrokeGroup) -> Self { Child::Group(group) }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Stroke {
    pub path: Path,
    pub typ:  Option<char>,
}

pub fn parse_kanjivg(root: roxmltree::Node) -> Vec<StrokeGroup> {
    assert_eq!(root.tag_name().name(), "kanjivg");

    let mut kanji_strokes = Vec::new();
    for node in root.children() {
        match (node.node_type(), node.tag_name().name()) {
            (NodeType::Element, "kanji") => {
                let id = node.attribute("id").unwrap();
                let unicode =
                    char::from_u32(u32::from_str_radix(&id[id.len() - 5..], 16).unwrap()).unwrap();

                let mut value = None;
                for node in node.children() {
                    match (node.node_type(), node.tag_name().name()) {
                        (NodeType::Element, "g") => {
                            assert!(value.is_none());
                            value = Some(parse_group(node));
                        }
                        (NodeType::Text, _) => (),
                        (ty, name) => panic!("Unexpected child in <kanji>: {:?} {}", ty, name),
                    }
                }
                let mut value = value.unwrap();
                value.element = Some(unicode);
                kanji_strokes.push(value);
            }
            (NodeType::Text, _) => (),
            (ty, name) => panic!("Unexpected child in <kanjivg>: {:?} {}", ty, name),
        }
    }
    kanji_strokes
}

fn parse_group(node: roxmltree::Node) -> StrokeGroup {
    assert_eq!(node.tag_name().name(), "g");

    let mut group = StrokeGroup::default();

    for attrib in node.attributes() {
        match attrib.name() {
            "id" => (),
            "element" => group.element = attrib.value().chars().next(),
            "variant" => match attrib.value() {
                "true" => group.variant = true,
                "false" => group.variant = false,
                _ => panic!("Unexpected value for variant: {}", attrib.value()),
            },
            "original" => group.original = attrib.value().chars().next(),
            "radical" => {
                group.radical = Some(attrib.value().parse().unwrap_or_else(|e| {
                    panic!("Unexpected value for radical: {} ({})", attrib.value(), e)
                }))
            }
            "position" => {
                group.position = Some(attrib.value().parse().unwrap_or_else(|e| {
                    panic!("Unexpected value for position: {} ({})", attrib.value(), e)
                }))
            }
            "part" => group.part = Some(attrib.value().parse().unwrap()),
            "number" => group.number = Some(attrib.value().parse().unwrap()),
            "partial" => {
                group.partial = match attrib.value() {
                    "true" => true,
                    "false" => false,
                    _ => panic!("Unexpected value for partial: {}", attrib.value()),
                }
            }
            "phon" => group.phon = attrib.value().chars().next(),
            "radicalForm" => {
                group.radical_form = match attrib.value() {
                    "true" => true,
                    "false" => false,
                    _ => panic!("Unexpected value for radicalForm: {}", attrib.value()),
                }
            }
            "tradForm" => {
                group.trad_form = match attrib.value() {
                    "true" => true,
                    "false" => false,
                    _ => panic!("Unexpected value for tradForm: {}", attrib.value()),
                }
            }
            _ => panic!("Unexpected attribute in <g>: {}", attrib.name()),
        }
    }

    for node in node.children() {
        match (node.node_type(), node.tag_name().name()) {
            (NodeType::Element, "path") => group.subgroups.push(parse_path(node).into()),
            (NodeType::Element, "g") => group.subgroups.push(parse_group(node).into()),
            (NodeType::Text, _) => (),
            (ty, name) => panic!("Unexpected child in <g>: {:?} {}", ty, name),
        }
    }

    group
}

fn parse_path(node: roxmltree::Node) -> Stroke {
    assert_eq!(node.tag_name().name(), "path");

    let mut stroke = Stroke {
        typ:  None,
        path: Path::default(),
    };

    for attrib in node.attributes() {
        match attrib.name() {
            "id" => (),
            "type" => stroke.typ = attrib.value().chars().next(),
            "d" => {
                stroke.path =
                    Path::parse_from_svg_path_data(attrib.value(), (0.0, 0.0, 109.0, 109.0))
            }
            _ => panic!("Unexpected attribute in <path>: {}", attrib.name()),
        }
    }
    for node in node.children() {
        match (node.node_type(), node.tag_name().name()) {
            (NodeType::Text, _) => (),
            (ty, name) => panic!("Unexpected child in <path>: {:?} {}", ty, name),
        }
    }

    stroke
}
