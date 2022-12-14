/// Takes the name of a struct, a visibility qualifier, a name. a type, and a field path, and
/// generates a setter, a getter, a mutter, and a mapper for that field
#[macro_export]
macro_rules! field {
    ($vis:vis $name:ident($($field:ident).+: $ty:ty)) => {
        paste::paste! {
            $vis fn $name(&mut self, $name: $ty) -> &mut Self {
                self.$($field).+ = $name;
                self
            }

            #[allow(dead_code)]
            $vis fn [<get_ $name>](&self) -> &$ty {
                &self.$($field).+
            }

            #[allow(dead_code)]
            $vis fn [<mut_ $name>](&mut self) -> &mut $ty {
                &mut self.$($field).+
            }

            #[allow(dead_code)]
            $vis fn [<map_ $name>](&mut self, f: impl FnOnce(&mut $ty)) -> &mut Self {
                f(&mut self.$($field).+);
                self
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct Builder {
        the_x: u32,
        s: String,
        inner: Inner,
    }

    #[derive(Debug, Default)]
    struct Inner {
        x: f32,
    }

    impl Builder {
        field!(pub x(the_x: u32));
        field!(pub(crate) s(s: String));
        field!(inner(inner.x: f32));
    }

    #[test]
    fn test() {
        let mut builder = Builder::default();

        builder.x(10);
        assert_eq!(builder.the_x, 10);

        builder.map_x(|x| *x *= 2);
        assert_eq!(builder.the_x, 20);

        builder.map_x(|x| *x *= 2);
        assert_eq!(builder.the_x, 40);

        *builder.mut_x() = 2048;
        assert_eq!(builder.the_x, 2048);

        assert_eq!(*builder.map_x(|x| *x *= 2).get_x(), 4096);
        assert_eq!(builder.the_x, 4096);

        builder.inner(2.0);
        assert_eq!(builder.inner.x, 2.0);

        builder.map_inner(|i| *i = (*i + 2.0) * 2.0);
        assert_eq!(*builder.get_inner(), 8.0);

        builder.s("Hello world".to_owned());
        assert_eq!(&builder.s, "Hello world");

        builder.map_s(|s| s.make_ascii_uppercase());
        assert_eq!(&builder.s, "HELLO WORLD");
    }
}
