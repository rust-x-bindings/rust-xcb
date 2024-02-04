(function() {var type_impls = {
"xcb":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-EnterNotifyEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1081-1253\">source</a><a href=\"#impl-EnterNotifyEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"xcb/x/struct.EnterNotifyEvent.html\" title=\"struct xcb::x::EnterNotifyEvent\">EnterNotifyEvent</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1082-1122\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.EnterNotifyEvent.html#tymethod.new\" class=\"fn\">new</a>(\n    detail: <a class=\"enum\" href=\"xcb/x/enum.NotifyDetail.html\" title=\"enum xcb::x::NotifyDetail\">NotifyDetail</a>,\n    time: <a class=\"type\" href=\"xcb/x/type.Timestamp.html\" title=\"type xcb::x::Timestamp\">Timestamp</a>,\n    root: <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a>,\n    event: <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a>,\n    child: <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a>,\n    root_x: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i16.html\">i16</a>,\n    root_y: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i16.html\">i16</a>,\n    event_x: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i16.html\">i16</a>,\n    event_y: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i16.html\">i16</a>,\n    state: <a class=\"struct\" href=\"xcb/x/struct.KeyButMask.html\" title=\"struct xcb::x::KeyButMask\">KeyButMask</a>,\n    mode: <a class=\"enum\" href=\"xcb/x/enum.NotifyMode.html\" title=\"enum xcb::x::NotifyMode\">NotifyMode</a>,\n    same_screen_focus: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a>\n) -&gt; <a class=\"struct\" href=\"xcb/x/struct.EnterNotifyEvent.html\" title=\"struct xcb::x::EnterNotifyEvent\">EnterNotifyEvent</a></h4></section><section id=\"method.response_type\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1128-1134\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.EnterNotifyEvent.html#tymethod.response_type\" class=\"fn\">response_type</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a></h4></section><section id=\"method.detail\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1136-1143\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.EnterNotifyEvent.html#tymethod.detail\" class=\"fn\">detail</a>(&amp;self) -&gt; <a class=\"enum\" href=\"xcb/x/enum.NotifyDetail.html\" title=\"enum xcb::x::NotifyDetail\">NotifyDetail</a></h4></section><section id=\"method.sequence\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1145-1151\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.EnterNotifyEvent.html#tymethod.sequence\" class=\"fn\">sequence</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u16.html\">u16</a></h4></section><section id=\"method.time\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1153-1159\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.EnterNotifyEvent.html#tymethod.time\" class=\"fn\">time</a>(&amp;self) -&gt; <a class=\"type\" href=\"xcb/x/type.Timestamp.html\" title=\"type xcb::x::Timestamp\">Timestamp</a></h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.root\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1162-1168\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.EnterNotifyEvent.html#tymethod.root\" class=\"fn\">root</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a></h4></section></summary><div class=\"docblock\"><p>The root window for the final cursor position.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.event\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1171-1177\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.EnterNotifyEvent.html#tymethod.event\" class=\"fn\">event</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a></h4></section></summary><div class=\"docblock\"><p>The window on which the event was generated.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.child\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1181-1187\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.EnterNotifyEvent.html#tymethod.child\" class=\"fn\">child</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a></h4></section></summary><div class=\"docblock\"><p>If the <code>event</code> window has subwindows and the final pointer position is in one\nof them, then <code>child</code> is set to that subwindow, <code>XCB_WINDOW_NONE</code> otherwise.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.root_x\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1190-1196\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.EnterNotifyEvent.html#tymethod.root_x\" class=\"fn\">root_x</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i16.html\">i16</a></h4></section></summary><div class=\"docblock\"><p>The pointer X coordinate relative to <code>root</code>’s origin at the time of the event.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.root_y\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1199-1205\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.EnterNotifyEvent.html#tymethod.root_y\" class=\"fn\">root_y</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i16.html\">i16</a></h4></section></summary><div class=\"docblock\"><p>The pointer Y coordinate relative to <code>root</code>’s origin at the time of the event.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.event_x\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1209-1215\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.EnterNotifyEvent.html#tymethod.event_x\" class=\"fn\">event_x</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i16.html\">i16</a></h4></section></summary><div class=\"docblock\"><p>If <code>event</code> is on the same screen as <code>root</code>, this is the pointer X coordinate\nrelative to the event window’s origin.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.event_y\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1219-1225\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.EnterNotifyEvent.html#tymethod.event_y\" class=\"fn\">event_y</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i16.html\">i16</a></h4></section></summary><div class=\"docblock\"><p>If <code>event</code> is on the same screen as <code>root</code>, this is the pointer Y coordinate\nrelative to the event window’s origin.</p>\n</div></details><section id=\"method.state\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1227-1234\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.EnterNotifyEvent.html#tymethod.state\" class=\"fn\">state</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/x/struct.KeyButMask.html\" title=\"struct xcb::x::KeyButMask\">KeyButMask</a></h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.mode\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1237-1244\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.EnterNotifyEvent.html#tymethod.mode\" class=\"fn\">mode</a>(&amp;self) -&gt; <a class=\"enum\" href=\"xcb/x/enum.NotifyMode.html\" title=\"enum xcb::x::NotifyMode\">NotifyMode</a></h4></section></summary><div class=\"docblock\"></div></details><section id=\"method.same_screen_focus\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1246-1252\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.EnterNotifyEvent.html#tymethod.same_screen_focus\" class=\"fn\">same_screen_focus</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a></h4></section></div></details>",0,"xcb::xproto::LeaveNotifyEvent"],["<section id=\"impl-Send-for-EnterNotifyEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1314\">source</a><a href=\"#impl-Send-for-EnterNotifyEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"xcb/x/struct.EnterNotifyEvent.html\" title=\"struct xcb::x::EnterNotifyEvent\">EnterNotifyEvent</a></h3></section>","Send","xcb::xproto::LeaveNotifyEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Raw%3Cxcb_generic_event_t%3E-for-EnterNotifyEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1066-1074\">source</a><a href=\"#impl-Raw%3Cxcb_generic_event_t%3E-for-EnterNotifyEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"xcb/trait.Raw.html\" title=\"trait xcb::Raw\">Raw</a>&lt;<a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_event_t.html\" title=\"struct xcb::ffi::xcb_generic_event_t\">xcb_generic_event_t</a>&gt; for <a class=\"struct\" href=\"xcb/x/struct.EnterNotifyEvent.html\" title=\"struct xcb::x::EnterNotifyEvent\">EnterNotifyEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1067-1069\">source</a><a href=\"#method.from_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">unsafe fn <a href=\"xcb/trait.Raw.html#tymethod.from_raw\" class=\"fn\">from_raw</a>(raw: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.pointer.html\">*mut </a><a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_event_t.html\" title=\"struct xcb::ffi::xcb_generic_event_t\">xcb_generic_event_t</a>) -&gt; Self</h4></section></summary><div class='docblock'>Build <code>Self</code> from a raw pointer <a href=\"xcb/trait.Raw.html#tymethod.from_raw\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1071-1073\">source</a><a href=\"#method.as_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"xcb/trait.Raw.html#tymethod.as_raw\" class=\"fn\">as_raw</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.pointer.html\">*mut </a><a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_event_t.html\" title=\"struct xcb::ffi::xcb_generic_event_t\">xcb_generic_event_t</a></h4></section></summary><div class='docblock'>Obtain the raw pointer representation</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/base.rs.html#67-71\">source</a><a href=\"#method.into_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"xcb/trait.Raw.html#method.into_raw\" class=\"fn\">into_raw</a>(self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.pointer.html\">*mut T</a></h4></section></summary><div class='docblock'>Convert self into a raw pointer <a href=\"xcb/trait.Raw.html#method.into_raw\">Read more</a></div></details></div></details>","Raw<xcb_generic_event_t>","xcb::xproto::LeaveNotifyEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BaseEvent-for-EnterNotifyEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1076-1079\">source</a><a href=\"#impl-BaseEvent-for-EnterNotifyEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"xcb/trait.BaseEvent.html\" title=\"trait xcb::BaseEvent\">BaseEvent</a> for <a class=\"struct\" href=\"xcb/x/struct.EnterNotifyEvent.html\" title=\"struct xcb::x::EnterNotifyEvent\">EnterNotifyEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.EXTENSION\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1077\">source</a><a href=\"#associatedconstant.EXTENSION\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"xcb/trait.BaseEvent.html#associatedconstant.EXTENSION\" class=\"constant\">EXTENSION</a>: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.75.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"enum\" href=\"xcb/enum.Extension.html\" title=\"enum xcb::Extension\">Extension</a>&gt; = None</h4></section></summary><div class='docblock'>The extension associated to this event, or <code>None</code> for the main protocol</div></details><details class=\"toggle\" open><summary><section id=\"associatedconstant.NUMBER\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1078\">source</a><a href=\"#associatedconstant.NUMBER\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"xcb/trait.BaseEvent.html#associatedconstant.NUMBER\" class=\"constant\">NUMBER</a>: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u32.html\">u32</a> = 7u32</h4></section></summary><div class='docblock'>The number associated to this event</div></details></div></details>","BaseEvent","xcb::xproto::LeaveNotifyEvent"],["<section id=\"impl-Sync-for-EnterNotifyEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1315\">source</a><a href=\"#impl-Sync-for-EnterNotifyEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"xcb/x/struct.EnterNotifyEvent.html\" title=\"struct xcb::x::EnterNotifyEvent\">EnterNotifyEvent</a></h3></section>","Sync","xcb::xproto::LeaveNotifyEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-EnterNotifyEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1306-1312\">source</a><a href=\"#impl-Drop-for-EnterNotifyEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"xcb/x/struct.EnterNotifyEvent.html\" title=\"struct xcb::x::EnterNotifyEvent\">EnterNotifyEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1307-1311\">source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/1.75.0/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","xcb::xproto::LeaveNotifyEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-EnterNotifyEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1255-1274\">source</a><a href=\"#impl-Debug-for-EnterNotifyEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"xcb/x/struct.EnterNotifyEvent.html\" title=\"struct xcb::x::EnterNotifyEvent\">EnterNotifyEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xproto.rs.html#1256-1273\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","xcb::xproto::LeaveNotifyEvent"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()