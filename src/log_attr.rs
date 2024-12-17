use serde::{ser::SerializeStruct, Serialize};

pub trait LogAttr: Serialize {
    const CHECK: usize = 0;

    fn add_attr<T: Serialize>(self, name: &'static str, value: T) -> impl LogAttr;

    fn serialize_attr<S: serde::ser::SerializeStruct>(&self, state: &mut S) -> Result<(), S::Error>;
}

struct NamedAttr<A: Serialize> {
    name: &'static str,
    value: A,
}

struct SerialAttr<P: LogAttr, N: Serialize> {
    prev: P,
    count: usize,
    next: NamedAttr<N>
}


impl LogAttr for () {
    fn add_attr<T: Serialize>(self, name: &'static str, value: T) -> impl LogAttr {
        NamedAttr { name, value }
    }

    fn serialize_attr<S: serde::ser::SerializeStruct>(&self, _: &mut S) -> Result<(), S::Error> {
        Ok(())
    }
}

impl<P: LogAttr, N: Serialize> Serialize for SerialAttr<P, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct("NestedSpanAttr", self.count)?;
        self.serialize_attr(&mut state)?;
        state.end()
    }
}

impl<A: Serialize> LogAttr for NamedAttr<A> {
    fn add_attr<T: Serialize>(self, name: &'static str, value: T) -> impl LogAttr {
        SerialAttr {
            prev: self,
            count: 2,
            next: NamedAttr { name, value }
        }
    }

    fn serialize_attr<S: serde::ser::SerializeStruct>(&self, state: &mut S) -> Result<(), S::Error> {
        state.serialize_field(self.name, &self.value)
    }
}

impl<A: LogAttr, B: Serialize> LogAttr for SerialAttr<A, B> {
    fn add_attr<T: Serialize>(self, name: &'static str, value: T) -> impl LogAttr {
        let count = self.count + 1;

        SerialAttr {
            prev: self,
            count,
            next: NamedAttr { name, value },
        }
    }

    fn serialize_attr<S: serde::ser::SerializeStruct>(&self, state: &mut S) -> Result<(), S::Error> {
        self.prev.serialize_attr(state)?;
        state.serialize_field(self.next.name, &self.next.value)
    }
}

impl<A: Serialize> Serialize for NamedAttr<A> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct("SpanAttr", 1)?;
        self.serialize_attr(&mut state)?;
        state.end()
    }
}

#[cfg(test)]
mod test {
    use super::LogAttr;

    #[test]
    fn test_attr() {
        let attr = ()
            .add_attr("hello world", "this is a test")
            .add_attr("what the heck", 32)
            .add_attr("yo", "foo")
        ;

        let result = serde_json::to_string(&attr).unwrap();
        assert_eq!(result, "{\"hello world\":\"this is a test\",\"what the heck\":32,\"yo\":\"foo\"}");
    }
}
