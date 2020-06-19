macro_rules! make_prop_enum_type {
    ($prop: ident, $propname: expr, $default: ident,
     $($ty: ident => $jprop: expr),*) => {
        #[derive(Copy, Clone, PartialEq, Debug)]
        pub enum $prop {
            $($ty,)*
        }

        impl Default for $prop {
            fn default() -> $prop { $prop::$default }
        }

        impl ToString for $prop {
            fn to_string(&self) -> String {
                match *self {
                    $($prop::$ty => $jprop,)*
                }.to_string()
            }
        }

        impl ToJson for $prop {
            fn to_json(&self) -> Json {
                Json::String(self.to_string())
            }
        }

        impl FromJson for $prop {
            fn from_json(json: &Json) -> Result<$prop,ParseError> {
                match *json {
                    Json::String(ref v) => match v.as_ref() {
                        $($jprop => Ok($prop::$ty),)*
                        _ => Err(ParseError::InvalidStructure($propname.to_string())),
                    },
                    _ => Err(ParseError::InvalidJsonType($propname.to_string())),
                }
            }
        }
    }
}

macro_rules! make_prop_type {
    ($prop: ident, $propname: expr,
     $($field: ident: $ty: ty => $jprop: expr),*) => {
        #[derive(Clone, PartialEq, Debug)]
        pub struct $prop {
            $($field: $ty,)*
        }

        impl Default for $prop {
            fn default() -> $prop {
                $prop {
                    $($field: Default::default()),*
                }
            }
        }

        impl ToJson for $prop {
            fn to_json(&self) -> Json {
                let mut d = BTreeMap::<String,Json>::new();
                $(self.$field.to_json_field(&mut d, $jprop);)*
                Json::Object(d)
            }
        }

        impl FromJson for $prop {
            fn from_json(json: &Json) -> Result<$prop,ParseError> {
                match *json {
                    Json::Object(ref o) => {
                        let mut prop = $prop::default();
                        $(prop.$field = FromJsonField::from_json_field(o, $jprop)?;)*
                        Ok(prop)
                    },
                    _ => Err(ParseError::InvalidJsonType($propname.to_string())),
                }
            }
        }
    }
}

macro_rules! make_record_type {
    ($record: ident, $partialrecord: ident, $recname: expr,
     $($field: ident: $ty: ty => $jprop: expr),*) => {
        #[derive(Clone, PartialEq, Debug)]
        pub struct $record {
            id: String,
            $($field: $ty),*
        }

        impl Default for $record {
            fn default() -> $record {
                $record {
                    id: record::new_id(),
                    $($field: Default::default()),*
                }
            }
        }

        impl ToJson for $record {
            fn to_json(&self) -> Json {
                let mut d = BTreeMap::<String,Json>::new();
                self.id.to_json_field(&mut d, "id");
                $(self.$field.to_json_field(&mut d, $jprop);)*
                Json::Object(d)
            }
        }

        impl FromJson for $record {
            fn from_json(json: &Json) -> Result<$record,ParseError> {
                match *json {
                    Json::Object(ref o) => {
                        let mut r = $record::default();
                        r.id = FromJsonField::from_json_field(o, "id")?;
                        $(r.$field = FromJsonField::from_json_field(o, $jprop)?;)*
                        Ok(r)
                    }
                    _ => Err(ParseError::InvalidJsonType($recname.to_string())),
                }
            }
        }


        #[derive(Clone, PartialEq, Debug)]
        pub struct $partialrecord {
            id: Presence<String>,
            $($field: Presence<$ty>),*
        }

        impl PartialRecord for $partialrecord {
            fn id(&self) -> Presence<String> {
                self.id.clone()
            }
        }

        impl Default for $partialrecord {
            fn default() -> $partialrecord {
                $partialrecord {
                    id: Absent,
                    $($field: Absent),*
                }
            }
        }

        impl ToJson for $partialrecord {
            fn to_json(&self) -> Json {
                let mut d = BTreeMap::<String,Json>::new();
                self.id.to_json_field(&mut d, "id");
                $(self.$field.to_json_field(&mut d, $jprop);)*
                Json::Object(d)
            }
        }

        impl FromJson for $partialrecord {
            fn from_json(json: &Json) -> Result<$partialrecord,ParseError> {
                match *json {
                    Json::Object(ref o) => {
                        let mut r = $partialrecord::default();
                        r.id = FromJsonField::from_json_field(o, "id")?;
                        $(r.$field = FromJsonField::from_json_field(o, $jprop)?;)*
                        Ok(r)
                    }
                    _ => Err(ParseError::InvalidJsonType($recname.to_string())),
                }
            }
        }


        impl Record for $record {
            type Partial = $partialrecord;

            fn id(&self) -> String {
                self.id.clone()
            }

            fn updated_with(&self, p: &$partialrecord) -> $record {
                let mut r = self.clone();
                let u = p.clone();
                if let Present(v) = u.id { r.id = v };
                $(if let Present(v) = u.$field { r.$field = v };)*
                r
            }

            fn to_partial(&self) -> $partialrecord {
                $partialrecord {
                    id: Present(self.id.clone()),
                    $($field: Present(self.$field.clone()),)*
                }
            }

            fn to_filtered_partial(&self, properties: &Vec<String>) -> $partialrecord {
                let mut p = $partialrecord::default();
                p.id = Present(self.id.clone());
                for prop in properties.iter() {
                    match prop.as_ref() {
                        $($jprop => p.$field = Present(self.$field.clone()),)*
                        _ => ()
                    }
                }
                p
            }
        }
    }
}

macro_rules! make_method_args_type {
    ($args: ident, $argsname: expr,
     $($field: ident: $ty: ty => $jprop: expr),*) => {
        #[derive(Clone, PartialEq, Debug)]
        pub struct $args {
            $(pub $field: $ty),*
        }

        impl Default for $args {
            fn default() -> $args {
                $args {
                    $($field: Default::default()),*
                }
            }
        }

        impl ToJson for $args {
            fn to_json(&self) -> Json {
                let mut d = BTreeMap::<String,Json>::new();
                $(self.$field.to_json_field(&mut d, $jprop);)*
                Json::Object(d)
            }
        }

        impl FromJson for $args {
            fn from_json(json: &Json) -> Result<$args,ParseError> {
                match *json {
                    Json::Object(ref o) => {
                        let mut args = <$args>::default();
                        $(args.$field = FromJsonField::from_json_field(o, $jprop)?;)*
                        Ok(args)
                    },
                    _ => Err(ParseError::InvalidJsonType($argsname.to_string())),
                }
            }
        }
    }
}

macro_rules! make_record_method_args_type {
    ($args: ident, $argsname: expr,
     $($field: ident: $ty: ty => $jprop: expr),*) => {
        #[derive(Clone, PartialEq, Debug)]
        pub struct $args<R> where R: Record {
            pub _marker: PhantomData<R>,
            $(pub $field: $ty),*
        }

        impl<R: Record> Default for $args<R> {
            fn default() -> $args<R> {
                $args::<R> {
                    _marker: PhantomData,
                    $($field: Default::default()),*
                }
            }
        }

        impl<R: Record> ToJson for $args<R> {
            fn to_json(&self) -> Json {
                let mut d = BTreeMap::<String,Json>::new();
                $(self.$field.to_json_field(&mut d, $jprop);)*
                Json::Object(d)
            }
        }

        impl<R: Record> FromJson for $args<R> {
            fn from_json(json: &Json) -> Result<$args<R>,ParseError> {
                match *json {
                    Json::Object(ref o) => {
                        let mut args = <$args<R>>::default();
                        $(args.$field = FromJsonField::from_json_field(o, $jprop)?;)*
                        Ok(args)
                    },
                    _ => Err(ParseError::InvalidJsonType($argsname.to_string())),
                }
            }
        }
    }
}

macro_rules! make_batch {
    ($batch: ident, $method: ty) => {
        #[derive(Clone, PartialEq, Debug)]
        pub struct $batch(pub Vec<$method>);

        impl Default for $batch {
            fn default() -> $batch {
                $batch(vec![])
            }
        }

        impl ToJson for $batch {
            fn to_json(&self) -> Json {
                self.0.to_json()
            }
        }

        impl FromJson for $batch {
            fn from_json(json: &Json) -> Result<$batch, ParseError> {
                match Vec::<$method>::from_json(json) {
                    Ok(v) => Ok($batch(v)),
                    Err(e) => Err(e),
                }
            }
        }
    };
}

macro_rules! make_methods {
    ($set: ident, $setname: expr, $error: ident, $($method: ident, $args: ty => $methodname: expr),*) => {
        #[derive(Clone, PartialEq, Debug)]
        pub enum $set {
            $($method($args, String),)*
        }

        impl ToJson for $set {
            fn to_json(&self) -> Json {
                Json::Array(
                    match *self {
                        $($method(ref args, ref client_id) =>
                            vec!($methodname.to_json(), args.to_json(), client_id.to_json()),)*
                    }
                )
            }
        }

        impl FromJson for $set {
            fn from_json(json: &Json) -> Result<$set,ParseError> {
                match *json {
                    Json::Array(ref a) => {
                        if let false = a.len() == 3 {
                            return Err(ParseError::InvalidStructure($setname.to_string()));
                        }
                        let method = String::from_json(&a[0])?;
                        let client_id = String::from_json(&a[2])?;
                        match method.as_ref() {
                            $($methodname => Ok($method(<$args>::from_json(&a[1])?, client_id)),)*
                            _ => Ok($error(MethodError::UnknownMethod(Present(ErrorDescription(method))), client_id)),
                        }
                    },
                    _ => Err(ParseError::InvalidJsonType($setname.to_string())),
                }
            }
        }

        impl ClientId for $set {
            fn client_id(&self) -> String {
                match *self {
                    $($method(_, ref id) => id,)*
                }.clone()
            }
        }
    }
}
