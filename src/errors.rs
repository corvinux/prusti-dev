use jni;
use std;

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain!{
    foreign_links {
        Io(std::io::Error);
        Utf8Error(std::str::Utf8Error);
        // FIXME: why is this required?
        UnknownJniError(jni::Error);
    }

    links {
        JniErrors(jni::errors::Error, jni::errors::ErrorKind);
    }

    errors {
        NoClass(class: String) {
            description("no class")
            display("no class '{}'", class)
        }

        NoConstructors(class: String) {
            description("no constructor")
            display("no constructors in class '{}'", class)
        }

        AmbiguousConstructor(class: String) {
            description("ambiguous constructor")
            display("ambiguous constructor in class '{}'", class)
        }

        NoMatchingConstructor(class: String, signature: String) {
            description("no matching constructor")
            display("no constructor in class '{}' with signature '{}'", class, signature)
        }

        NoMethod(class: String, method: String) {
            description("no method")
            display("no method '{}' in class '{}'", method, class)
        }

        AmbiguousMethod(class: String, method: String) {
            description("ambiguous method")
            display("ambiguous method '{}' in class '{}'", method, class)
        }

        NoMatchingMethod(class: String, method: String, signature: String) {
            description("no matching method")
            display("no method '{}' with signature '{}' in class '{}'", method, signature, class)
        }
    }
}
