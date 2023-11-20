use imgui_presentable::*;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

fn check_that_implements<T: ImguiPresentable>(_object: T) {}

mod imgui_presentation_is_properly_derived_for {
    use super::*;

    #[derive(Default, ImguiPresentation)]
    struct ScalarsStruct {
        value_i8: i8,
        value_u8: u8,
        value_i16: i16,
        value_u16: u16,
        value_i32: i32,
        value_u32: u32,
        value_i64: i64,
        value_u64: u64,
        value_isize: isize,
        value_usize: usize,
        value_f32: f32,
        value_f64: f64,
        value_bool: bool,
    }

    #[derive(Default, ImguiPresentation)]
    struct ScalarsTuple(
        i8,
        u8,
        i16,
        u16,
        i32,
        u32,
        i64,
        u64,
        f32,
        f64,
        isize,
        usize,
        bool,
    );

    #[derive(Default, ImguiPresentation)]
    struct StringsOwned {
        string_owned: String,
    }
    #[derive(Default, ImguiPresentation)]
    struct StringsOwnedTuple(String, String);

    // TODO: work with mutable/immutable references to objects.
    // #[derive(Default, ImguiPresentation)]
    // struct StringsSlice<'a> {
    //     string_slice: &'a mut str,
    // }

    #[derive(Default, ImguiPresentation)]
    struct Sequential {
        vector_i8: Vec<i8>,
        btree_set: BTreeSet<String>,
        hash_set: HashSet<String>,
        // TODO: work on arrays and matreshkas.
        // array_i8: [i8; 10],
        // vector_vector_string: Vec<Vec<String>>,
    }

    #[derive(Default, ImguiPresentation)]
    struct Maps {
        btree_map: BTreeMap<i8, String>,
        hash_map: HashMap<i8, String>,
    }

    #[derive(Default, ImguiPresentation)]
    struct InnerTypes {
        inner_maps: Maps,
    }

    #[derive(Default, ImguiPresentation)]
    struct InnerTypes2 {
        inner_maps: BTreeMap<String, InnerTypes>,
    }

    #[derive(Default, ImguiPresentation)]
    struct StrictType(u16);

    #[derive(Default, ImguiPresentation)]
    struct InnerTypes3 {
        field: StrictType,
    }

    #[derive(ImguiPresentation)]
    enum PodEnum1 {
        Variant1,
        Variant2,
        Variant3,
    }

    #[derive(ImguiPresentation)]
    enum PodEnum2 {
        Variant1 = 2,
        Variant2 = 4,
        Variant3 = 1,
        Variant4 = 100,
    }

    #[derive(Default, ImguiPresentation)]
    struct Options {
        option_string: Option<String>,
    }

    /// Doc-comment
    #[derive(Default, ImguiPresentation)]
    pub struct CollectionIsNotUsedClippy {
        pub a: Option<f32>,
        pub b: Option<String>,
    }

    #[test]
    fn scalar_types() {
        let s = ScalarsStruct::default();
        check_that_implements(s);
        let s = ScalarsTuple::default();
        check_that_implements(s);
    }

    #[test]
    fn strings() {
        let s = StringsOwned::default();
        check_that_implements(s);
        let s = StringsOwnedTuple::default();
        check_that_implements(s);
    }

    #[test]
    fn sequential() {
        let s = Sequential::default();
        check_that_implements(s);
    }

    #[test]
    fn maps() {
        let m = Maps::default();
        check_that_implements(m);
    }

    #[test]
    fn inner_types() {
        let e = InnerTypes::default();
        check_that_implements(e);
        let e = InnerTypes2::default();
        check_that_implements(e);
        let e = InnerTypes3::default();
        check_that_implements(e);
    }

    #[test]
    fn pod_enums() {
        let e = PodEnum1::Variant1;
        check_that_implements(e);
        let e = PodEnum2::Variant1;
        check_that_implements(e);
    }

    #[test]
    fn options() {
        let e = Options::default();
        check_that_implements(e);
    }

    #[test]
    fn collection_is_not_used() {
        let e = CollectionIsNotUsedClippy::default();
        check_that_implements(e);
    }
}

mod imgui_presentation_attributes {
    use super::*;

    #[derive(Default, ImguiPresentation)]
    struct SkipAndReadOnlyFields {
        do_not_skip_field: bool,
        #[imgui_presentation(skip)]
        _skip_field: bool,
        #[imgui_presentation(readonly)]
        readonly_field: bool,
        #[imgui_presentation(skip, readonly)]
        _skip_readonly_field: bool,
    }

    #[derive(Default, ImguiPresentation)]
    #[imgui_presentation(readonly)]
    struct ReadOnlyStruct {
        value: f32,
    }

    #[derive(Default, ImguiPresentation)]
    struct StructWithRenamedField {
        #[imgui_presentation(rename = "not_value")]
        value: bool,
    }

    #[derive(Default, ImguiPresentation)]
    struct StructWithFormattedField {
        #[imgui_presentation(format = "%.2f dollars")]
        value: f32,
    }

    #[derive(Default, ImguiPresentation)]
    struct StructWithDocCommentField {
        /// This is the way.
        #[imgui_presentation(format = "%.2f dollars")]
        value: f32,
    }

    #[derive(Default, ImguiPresentation)]
    #[imgui_presentation(tooltip = "asd")]
    struct StructWithToolTip {
        value: f32,
    }

    #[derive(Default, ImguiPresentation)]
    #[imgui_presentation(button("Hello world": "on_hello_world"))]
    #[imgui_presentation(button("Hello world 2": "on_hello_world_2"))]
    struct StructWithButtons {
        value: f32,
    }

    impl StructWithButtons {
        fn on_hello_world(&mut self) {}
        fn on_hello_world_2(&mut self) {}
    }

    #[test]
    fn skip_and_readonly_fields() {
        let e = SkipAndReadOnlyFields::default();
        check_that_implements(e);
    }

    #[test]
    fn readonly_struct() {
        let e = ReadOnlyStruct::default();
        check_that_implements(e);
    }

    #[test]
    fn struct_with_formatted_field() {
        let e = StructWithFormattedField::default();
        check_that_implements(e);
    }

    #[test]
    fn struct_with_doc_comment_field() {
        let e = StructWithDocCommentField::default();
        check_that_implements(e);
    }

    #[test]
    fn struct_with_tooltip() {
        let e = StructWithToolTip::default();
        check_that_implements(e);
    }
}
