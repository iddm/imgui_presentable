use std::{collections::HashSet, str::FromStr};

use quote::{quote, ToTokens};

use crate::Backend;

pub type Result<T = (), E = proc_macro2::TokenStream> = std::result::Result<T, E>;

fn parse_button_declaration(input: &str) -> Button {
    // TODO just use the Punctuated.
    let regex = regex::Regex::new(r#"^\s*\"(.*)\"\s*:\s*\"(.*)\"\s*$"#).unwrap();
    let mut captures = regex.captures_iter(input);
    let captured: (_, [&str; 2]) = captures.next().expect("Parse the button.").extract();
    Button {
        title: captured
            .1
            .first()
            .expect("Parse the button title.")
            .to_string(),
        method_name: captured
            .1
            .get(1)
            .expect("Parse the button method name.")
            .to_string(),
    }
}

fn parse_main_menu_item_declaration(input: &str) -> MainMenuItem {
    // TODO just use the Punctuated.
    let regex = regex::Regex::new(r#"^\s*\"(.*)\"\s*:\s*\"(.*)\"\s*$"#).unwrap();
    let mut captures = regex.captures_iter(input);
    let captured: (_, [&str; 2]) = captures
        .next()
        .expect("Parse the main menu item.")
        .extract();
    MainMenuItem {
        title: captured
            .1
            .first()
            .expect("Parse the main menu item title.")
            .to_string(),
        method_name: captured
            .1
            .get(1)
            .expect("Parse the main menu item method name.")
            .to_string(),
        hot_key: None,
    }
}

/// A button with title and the method name which should be called on
/// [`self`] (for the [`ImguiPresentable`] object) once the button is
/// pushed.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Button {
    /// The title of the button.
    pub title: String,
    /// The method name to call on `self` once the button is clicked.
    pub method_name: String,
}

/// A menu item with title and the method name which should be called on
/// [`self`] (for the [`ImguiPresentable`] object) once the button is
/// pushed.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MainMenuItem {
    /// The title of the button.
    pub title: String,
    /// The method name to call on `self` once the button is clicked.
    pub method_name: String,
    /// The hotkey combination which can also trigger the menu item.
    pub hot_key: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Attribute {
    /// Skips generating the code for the marked field.
    Skip,
    /// Overrides the read only by not providing the mutable
    /// implementation.
    ReadOnly,
    /// Rename the field for imgui representation.
    Rename(String),
    /// Set the display format for the field (printf syntax).
    Format(String),
    /// Allows to have a tooltip text that overrides the documentation.
    Tooltip(String),
    /// The documentation string.
    Documentation(String),
    /// Buttons for the struct to add below all the imgui generated
    /// code. Any custom logic may be performed there. The buttons may
    /// only be called in the mutable presentation mode, when the object
    /// can be changed (perhaps, this will change in the future).
    Button(Button),
    /// Allows to select a backend.
    Backend(Backend),
    /// A main menu item.
    MainMenuItem(MainMenuItem),
}

impl FromStr for Attribute {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // If an attribute has value.
        if input.contains('=') {
            let (attribute, value) = {
                let mut split = input.split('=');
                let attribute = split.next().unwrap().trim().to_lowercase();
                let value = split
                    .next()
                    .unwrap_or_else(|| panic!("A value for {attribute} attribute is required."))
                    .trim()
                    .replace('"', "")
                    .to_owned();
                (attribute, value)
            };
            Ok(match attribute.as_ref() {
                "rename" => Self::Rename(value),
                "format" => Self::Format(value),
                "tooltip" => Self::Tooltip(value),
                "backend" => Self::Backend(Backend::from_str(&value)?),
                a => return Err(a.to_owned()),
            })
        } else if input.contains('(') {
            let (attribute, value) = {
                let mut split = input.split('(');
                let attribute = split.next().unwrap().trim().to_lowercase();
                let value = split
                    .next()
                    .unwrap_or_else(|| panic!("A value for {attribute} attribute is required."))
                    .trim()
                    .replace([')'], "")
                    .to_owned();
                (attribute, value)
            };
            Ok(match attribute.as_ref() {
                "button" => {
                    let button = parse_button_declaration(&value);
                    Self::Button(button)
                }
                "main_menu_item" => {
                    let main_menu_item = parse_main_menu_item_declaration(&value);
                    Self::MainMenuItem(main_menu_item)
                }
                a => return Err(a.to_owned()),
            })
        } else {
            // Attributes without a value.
            Ok(match input.trim().to_lowercase().as_ref() {
                "skip" => Self::Skip,
                "readonly" => Self::ReadOnly,
                a => return Err(a.to_owned()),
            })
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Attributes {
    attributes: HashSet<Attribute>,
}

impl From<HashSet<Attribute>> for Attributes {
    fn from(attributes: HashSet<Attribute>) -> Self {
        Self { attributes }
    }
}

impl std::ops::Deref for Attributes {
    type Target = HashSet<Attribute>;

    fn deref(&self) -> &Self::Target {
        &self.attributes
    }
}

impl std::ops::DerefMut for Attributes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attributes
    }
}

/// Allows an easier access to the documentation part of the attribute.
pub trait AttributeHasDocumentation {
    fn has_documentation(&self) -> bool;
    fn get_documentation_name_value(&self) -> Option<&syn::MetaNameValue>;
    fn get_documentation(&self) -> Option<&syn::Expr>;
    fn get_documentation_string(&self) -> Option<String>;
}

/// Allows an easier access the meta list.
pub trait AttributeHasMetaList {
    fn get_meta_list(&self) -> Option<&syn::MetaList>;
}

impl AttributeHasDocumentation for syn::Attribute {
    fn has_documentation(&self) -> bool {
        self.get_documentation_name_value().is_some()
    }

    fn get_documentation_name_value(&self) -> Option<&syn::MetaNameValue> {
        match &self.meta {
            syn::Meta::NameValue(name_value) => {
                if name_value.path.to_token_stream().to_string() == "doc" {
                    Some(name_value)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn get_documentation(&self) -> Option<&syn::Expr> {
        self.get_documentation_name_value()
            .map(|name_value| &name_value.value)
    }

    fn get_documentation_string(&self) -> Option<String> {
        self.get_documentation().and_then(|expr| match expr {
            syn::Expr::Lit(lit) => match &lit.lit {
                syn::Lit::Str(str) => Some(str.value()),
                _ => None,
            },
            _ => None,
        })
    }
}

impl AttributeHasMetaList for syn::Attribute {
    fn get_meta_list(&self) -> Option<&syn::MetaList> {
        match &self.meta {
            syn::Meta::List(list) => Some(list),
            _ => None,
        }
    }
}

impl Attributes {
    pub fn new(attributes: &[String]) -> Result<Self> {
        let mut unknown_attributes = Vec::new();
        let attributes = attributes
            .iter()
            .filter_map(|a| {
                if let Ok(a) = Attribute::from_str(a) {
                    Some(a)
                } else {
                    unknown_attributes.push(a);
                    None
                }
            })
            .collect();

        if unknown_attributes.is_empty() {
            Ok(Self { attributes })
        } else {
            let attrs = unknown_attributes
                .into_iter()
                .map(|s| format!("#[imgui_presentation]: unknown attributes: {s}"))
                .collect::<Vec<String>>()
                .join("\n");

            Err(quote! {
                compile_error!(#attrs);
            })
        }
    }

    pub fn parse(attribute: &syn::Attribute) -> Result<Self> {
        // println!("Parsing attribute: {}", attribute.meta.to_token_stream());

        let docs = attribute.get_documentation_string().map(|value| {
            let mut attributes = HashSet::new();
            attributes.insert(Attribute::Documentation(value.to_owned()));
            Self { attributes }
        });

        if let Some(docs) = docs {
            return Ok(docs);
        }

        let list = attribute.get_meta_list().unwrap();

        let our_attribute = list.path.to_token_stream().to_string();
        if our_attribute != "imgui_presentation" {
            return Ok(Self::default());
        }

        let strings: Vec<String> = list
            .tokens
            .to_token_stream()
            .to_string()
            .split(',')
            .map(|s| s.to_owned())
            .collect();

        Self::new(&strings)
    }

    pub fn parse_many(attributes: &[syn::Attribute]) -> Result<Self> {
        let attributes: HashSet<Attribute> = attributes
            .iter()
            .map(Self::parse)
            // .flat_map(|a| a.unwrap().attributes)
            .collect::<Result<Vec<Attributes>>>()?
            .into_iter()
            .flat_map(|a| a.attributes)
            .collect();
        Ok(attributes.into())
    }

    pub fn parse_from_field(field: &syn::Field) -> Result<Self> {
        Self::parse_many(&field.attrs)
    }

    pub fn has_skip(&self) -> bool {
        self.attributes.contains(&Attribute::Skip)
    }

    pub fn has_readonly(&self) -> bool {
        self.attributes.contains(&Attribute::ReadOnly)
    }

    pub fn get_rename(&self) -> Option<&str> {
        self.attributes.iter().find_map(|a| {
            if let Attribute::Rename(s) = a {
                Some(s.as_ref())
            } else {
                None
            }
        })
    }

    // TODO Unused for now, will be used in the future.
    #[allow(dead_code)]
    pub fn get_format(&self) -> Option<&str> {
        self.attributes.iter().find_map(|a| {
            if let Attribute::Format(s) = a {
                Some(s.as_ref())
            } else {
                None
            }
        })
    }

    pub fn get_documentation(&self) -> Option<String> {
        // There might be many, due to how Rust creates those.
        let strings = self
            .attributes
            .iter()
            .filter_map(|a| {
                if let Attribute::Documentation(s) = a {
                    Some(s.trim())
                } else {
                    None
                }
            })
            .collect::<Vec<&str>>();

        if strings.is_empty() {
            None
        } else {
            Some(strings.join(""))
        }
    }

    pub fn get_tooltip(&self) -> Option<&str> {
        self.attributes.iter().find_map(|a| {
            if let Attribute::Tooltip(s) = a {
                Some(s.as_ref())
            } else {
                None
            }
        })
    }

    pub fn get_buttons(&self) -> Vec<&Button> {
        self.attributes
            .iter()
            .filter_map(|a| {
                if let Attribute::Button(s) = a {
                    Some(s)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_main_menu_items(&self) -> Vec<&MainMenuItem> {
        self.attributes
            .iter()
            .filter_map(|a| {
                if let Attribute::MainMenuItem(s) = a {
                    Some(s)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_backends(&self) -> Vec<&Backend> {
        self.attributes
            .iter()
            .filter_map(|a| {
                if let Attribute::Backend(b) = a {
                    Some(b)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_tooltip_or_documentation(&self) -> Option<String> {
        self.get_tooltip()
            .map(|s| s.to_owned())
            .or(self.get_documentation())
    }
}
