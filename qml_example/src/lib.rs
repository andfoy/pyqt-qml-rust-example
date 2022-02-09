extern crate cstr;

use std::ffi::CStr;
use std::os::raw::{c_void};

use qmetaobject::prelude::*;
use qmetaobject::{QObjectCppWrapper, QQuickPaintedItem, QMouseEvent, QObjectDescriptor, QObjectPinned, QPainter, PropertyType, QMetaType};

use cpp::{cpp, cpp_class};
use cstr::cstr;

cpp! {{
    #include <QtWidgets/QPlainTextEdit>
    #include <QtQuick/QQuickPaintedItem>
    #include <QtQuick/QQuickItem>
    #include <QtGui/QKeyEvent>
    #include <QtCore/QPointer>
    #include <QtQuick/QQuickPaintedItem>
    #include <QtQuick/QQuickItem>
    #include <stdint.h>
}}

/// Only a specific subset of [`QEvent::Type`][qt] enum.
///
/// [qt]: https://doc.qt.io/qt-5/qevent.html#Type-enum
#[repr(C)]
#[non_exhaustive]
pub enum QEventType {
    None = 0,
    ActionAdded = 114,
    ActionChanged = 113,
}


/// A reference to a [`QMouseEvent`][qt] instance.
///
/// [qt]: https://doc.qt.io/qt-5/qmouseevent.html
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct QEvent(*const c_void, std::marker::PhantomData<u32>);

impl QEvent {
    /// Returns the type of event
    pub fn event_type(self) -> QEventType {
        cpp!(unsafe [self as "QEvent *"] -> QEventType as "int" {
            return self->type();
        })
    }

    /// Accept the event
    pub fn accept(self) {
        cpp!(unsafe [self as "QEvent *"] {
            self->accept();
        })
    }

    pub fn create_event(event: QEventType) -> QEvent {
        let mut ev = QEvent(std::ptr::null(), std::marker::PhantomData{});
        let mut ev_0 = ev.0;
        cpp!(unsafe [mut ev_0 as "QEvent*", event as "int"] -> QEvent as "QEvent*"{
            QEvent q_event(static_cast<QEvent::Type>(event));
            ev_0 = &q_event;
            return ev_0;
        })
    }
}

impl Default for QEvent {
    fn default() -> QEvent {
        QEvent::create_event(QEventType::None)
    }
}

impl QMetaType for QEvent {

}


/// Only a specific subset of [`QEvent::Type`][qt] enum.
///
/// [qt]: https://doc.qt.io/qt-5/qevent.html#Type-enum
#[repr(C)]
#[non_exhaustive]
pub enum QKeyboardModifiers {
    NoModifier = 0x00000000,
    ShiftModifier = 0x02000000,
    ControlModifier = 0x04000000,
    AltModifier = 0x08000000,
    MetaModifier = 0x10000000,
    KeypadModifier = 0x20000000,
    GroupSwitchModifier = 0x40000000,
}



/// A reference to a [`QMouseEvent`][qt] instance.
///
/// [qt]: https://doc.qt.io/qt-5/qmouseevent.html
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct QKeyEvent<'a>(*const c_void, std::marker::PhantomData<&'a u32>);

impl<'a> QKeyEvent<'a> {
    /// Return the number of keys involved in the event
    pub fn count(&self) -> i32 {
        cpp!(unsafe [self as "QKeyEvent *"] -> i32 as "int" {
            return self->count();
        })
    }

    /// Returns true if this event comes from an auto-repeating key; returns false if it comes from an initial key press.
    pub fn is_auto_repeat(&self) -> bool {
        cpp!(unsafe [self as "QKeyEvent *"] -> bool as "bool" {
            return self->isAutoRepeat();
        })
    }

    /// Returns the Unicode text that this key generated.
    pub fn text(&self) -> QString {
        cpp!(unsafe [self as "QKeyEvent *"] -> QString as "QString" {
            return self->text();
        })
    }

}



pub trait QWidget: QObject {
    fn get_object_description() -> &'static QObjectDescriptor
    where
        Self: Sized,
    {
        unsafe {
            &*cpp!([]-> *const QObjectDescriptor as "RustQObjectDescriptor const*" {
                return RustQObjectDescriptor::instance<Rust_QWidget>();
            })
        }
    }

    fn event(&mut self, _event: QEvent) -> bool {
        false
    }

    fn keyPressEvent(&mut self, _event: QKeyEvent) {

    }
}

cpp! {{
    #include <qmetaobject_rust.hpp>
    #include <QtWidgets/QWidget>

    struct Rust_QWidget : RustObject<QWidget> {
        bool event(QEvent* ev) override {
            bool filter = rust!(Rust_QEvent_event[
                rust_object: QObjectPinned<dyn QWidget> as "TraitObject",
                ev: QEvent as "QEvent *"
            ] -> bool as "bool" {
                rust_object.borrow_mut().event(ev)
            });

            if (!filter) {
                filter = QWidget::event(ev);
            }

            return filter;
            // if (!) { ev->ignore(); }
        }

        void keyPressEvent(QKeyEvent* ev) override {
            rust!(Rust_QEvent_keyPressEvent[
                rust_object: QObjectPinned<dyn QWidget> as "TraitObject",
                ev: QKeyEvent as "QKeyEvent *"
            ] {
                rust_object.borrow_mut().keyPressEvent(ev)
            });

            QWidget::keyPressEvent(ev);
        }
    };
}}

impl<'a> dyn QWidget + 'a {

}

cpp! {{
    // struct QWidgetRedirector: QWidget {
    //     void keyPressEvent(QKeyEvent* event);
    // };

    struct QPlainTextExtended: QPlainTextEdit {
        QPlainTextExtended(QWidget *parent = nullptr): QPlainTextEdit(parent) {};

        void keyPressEvent(QKeyEvent* event) override {
            auto parent = (QWidget*) this->parent();

            if (parent != nullptr) {
                // Subclass QWidget so that we can call protected functions
                class Hack : public QWidget
                {
                public:
                    using QWidget::keyPressEvent;
                };

                static_cast<Hack *>(parent)->keyPressEvent(event);
                // parent->keyPressEvent(event);
            }

            QPlainTextEdit::keyPressEvent(event);
        }
    };

}}


#[derive(QObject)]
struct QPlainTextExtended {
    base: qt_base_class!(trait QWidget)
}

impl QWidget for QPlainTextExtended {}

impl QPlainTextExtended {
    pub fn new(parent: *const c_void) -> QPlainTextExtended {
        let mut ptr: *mut c_void = std::ptr::null_mut();
        cpp!(unsafe [mut ptr as "void*", parent as "QWidget*"] {
            QObject* editor = new QPlainTextExtended(parent);
            ptr = editor;
        });

        let mut cpp_ptr: QObjectCppWrapper = QObjectCppWrapper::default();
        cpp_ptr.set(ptr);
        QPlainTextExtended {
            base: cpp_ptr
        }
    }
}



#[derive(QObject, Default)]
struct QmlPlainTextEdit {
    base: qt_base_class!(trait QQuickPaintedItem),
    editor: qt_property!(QPointer<QPlainTextExtended>; WRITE set_editor),
    create_editor: qt_method!(fn(&mut self)),
    event: qt_method!(fn(&mut self, event: QEvent) -> bool)
    // keyPressEvent: qt_method!(fn(&self, event: QEvent))
}

impl QmlPlainTextEdit {
    pub fn create_editor(&mut self) {
        let obj = self.get_cpp_object();
        assert!(!obj.is_null());

        let qplain_text = QPlainTextExtended::new(obj);
        let qplain_obj = qplain_text.get_cpp_object();
        assert!(!qplain_obj.is_null());

        cpp!(unsafe [qplain_obj as "QPlainTextExtended*", obj as "QQuickPaintedItem*"] {
            obj -> setFlag(QQuickItem::ItemHasContents, true);
            obj -> setFlag(QQuickItem::ItemAcceptsInputMethod, true);
            obj -> setFlag(QQuickItem::ItemIsFocusScope, true);
            qplain_obj -> installEventFilter(obj);
            auto hint = qplain_obj->sizeHint();

            class Hack : public QQuickPaintedItem
            {
                public:
                    using QQuickItem::setImplicitSize;
            };

            static_cast<Hack*>(obj) -> setImplicitSize(hint.width(), hint.height());
            // textEdit()->setVerticalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
            qplain_obj->setSizeAdjustPolicy(QPlainTextEdit::AdjustToContents);
        });

        let ptr: QPointer<QPlainTextExtended> = QPointer::from(&qplain_text);
        self.set_editor(ptr);


    }

    pub fn set_editor(&mut self, editor: QPointer<QPlainTextExtended>) {
        self.editor = editor;
    }

    pub fn event(&mut self, event: QEvent) -> bool {
        true
    }
}

impl QQuickItem for QmlPlainTextEdit {

}

impl QQuickPaintedItem for QmlPlainTextEdit {
    fn paint(&mut self, p: &mut QPainter) {
        // let painter_ref = self.get_cpp_object();
        let editor_ref = self.editor.as_ref().unwrap();
        let editor_ptr = editor_ref.get_cpp_object();

        cpp!(unsafe [editor_ptr as "QPlainTextExtended*", p as "QPainter*"] {
            editor_ptr -> render(p);
        })
    }
}



#[derive(Default, QObject)]
struct QExampleQmlPlugin {
    base: qt_base_class!(trait QQmlExtensionPlugin),
    plugin: qt_plugin!("org.qt-project.Qt.QQmlExtensionInterface/1.0"),
}

impl QQmlExtensionPlugin for QExampleQmlPlugin {
    fn register_types(&mut self, uri: &CStr) {
        //assert_eq!(uri, cstr!("TimeExample"));
        qml_register_type::<QmlPlainTextEdit>(uri, 1, 0, cstr!("PlainTextEdit"));
    }
}

// #[derive(QObject)]
// struct QmlPlainTextEdit {
//     base: qt_base_class!(trait QQuickPaintedItem),
//     editor: qt_property!(QPointer<QPlainTextEdit>)
// }

// impl QQuickItem for QmlPlainTextEdit {
//     fn mouse_event(&mut self, event: QMouseEvent) {

//     }
// }

// impl QQuickPaintedItem for QmlPlainTextEdit {

// }



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
