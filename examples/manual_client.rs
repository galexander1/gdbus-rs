/*
 * Copyright (c) 2016 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

extern crate gdbus;
extern crate gtk;

use gdbus::connection::{Connection, SEND_MESSAGE_FLAGS_NONE};
use gdbus::message::Message;
use gdbus::own_name::Type;
use gdbus::variant::FromVariant;
use gdbus::watch::{Watch, NAME_WATCHER_FLAGS_NONE};

fn on_name_appeared(connection: &Connection, name_owner: &str) {
    let method_call_message = Message::new_method_call(name_owner, "/org/gtk/GDBus/TestObject", "org.gtk.GDBus.TestInterface", "decrement_increment");
    method_call_message.set_body((41i64,));
    match connection.send_message_with_reply_sync(method_call_message, SEND_MESSAGE_FLAGS_NONE) {
        Ok(message) => {
            let (decrement, increment): (i64, u8) = FromVariant::from_variant(&message.get_body());
            println!("Response: ({}, {})", decrement, increment);
        },
        Err(error) => println!("Error: {}", error),
    }
}

fn main() {
    gtk::init().unwrap();

    let _watcher = Watch::name(Type::Session, "org.gtk.GDBus.TestServer", NAME_WATCHER_FLAGS_NONE)
        .connect_name_appeared(on_name_appeared)
        .build();

    gtk::main();
}
