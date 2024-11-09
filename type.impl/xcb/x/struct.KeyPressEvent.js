(function() {
    var type_impls = Object.fromEntries([["xcb",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BaseEvent-for-KeyPressEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#362-365\">source</a><a href=\"#impl-BaseEvent-for-KeyPressEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"xcb/trait.BaseEvent.html\" title=\"trait xcb::BaseEvent\">BaseEvent</a> for <a class=\"struct\" href=\"xcb/x/struct.KeyPressEvent.html\" title=\"struct xcb::x::KeyPressEvent\">KeyPressEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.EXTENSION\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#363\">source</a><a href=\"#associatedconstant.EXTENSION\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"xcb/trait.BaseEvent.html#associatedconstant.EXTENSION\" class=\"constant\">EXTENSION</a>: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"enum\" href=\"xcb/enum.Extension.html\" title=\"enum xcb::Extension\">Extension</a>&gt; = None</h4></section></summary><div class='docblock'>The extension associated to this event, or <code>None</code> for the main protocol</div></details><details class=\"toggle\" open><summary><section id=\"associatedconstant.NUMBER\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#364\">source</a><a href=\"#associatedconstant.NUMBER\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"xcb/trait.BaseEvent.html#associatedconstant.NUMBER\" class=\"constant\">NUMBER</a>: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u32.html\">u32</a> = 2u32</h4></section></summary><div class='docblock'>The number associated to this event</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_from_send_event\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/base.rs.html#87-91\">source</a><a href=\"#method.is_from_send_event\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"xcb/trait.BaseEvent.html#method.is_from_send_event\" class=\"fn\">is_from_send_event</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Check whether this event was emitted by the SendEvent request\nSee <code>[crate::x::SendEvent]</code>.</div></details></div></details>","BaseEvent","xcb::xproto::KeyReleaseEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-KeyPressEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#529-548\">source</a><a href=\"#impl-Debug-for-KeyPressEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"xcb/x/struct.KeyPressEvent.html\" title=\"struct xcb::x::KeyPressEvent\">KeyPressEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#530-547\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","xcb::xproto::KeyReleaseEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-KeyPressEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#580-586\">source</a><a href=\"#impl-Drop-for-KeyPressEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"xcb/x/struct.KeyPressEvent.html\" title=\"struct xcb::x::KeyPressEvent\">KeyPressEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#581-585\">source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/1.82.0/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","xcb::xproto::KeyReleaseEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-KeyPressEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#367-527\">source</a><a href=\"#impl-KeyPressEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"xcb/x/struct.KeyPressEvent.html\" title=\"struct xcb::x::KeyPressEvent\">KeyPressEvent</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#368-405\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.KeyPressEvent.html#tymethod.new\" class=\"fn\">new</a>(\n    detail: <a class=\"type\" href=\"xcb/x/type.Keycode.html\" title=\"type xcb::x::Keycode\">Keycode</a>,\n    time: <a class=\"type\" href=\"xcb/x/type.Timestamp.html\" title=\"type xcb::x::Timestamp\">Timestamp</a>,\n    root: <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a>,\n    event: <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a>,\n    child: <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a>,\n    root_x: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.i16.html\">i16</a>,\n    root_y: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.i16.html\">i16</a>,\n    event_x: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.i16.html\">i16</a>,\n    event_y: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.i16.html\">i16</a>,\n    state: <a class=\"struct\" href=\"xcb/x/struct.KeyButMask.html\" title=\"struct xcb::x::KeyButMask\">KeyButMask</a>,\n    same_screen: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.bool.html\">bool</a>,\n) -&gt; <a class=\"struct\" href=\"xcb/x/struct.KeyPressEvent.html\" title=\"struct xcb::x::KeyPressEvent\">KeyPressEvent</a></h4></section><section id=\"method.response_type\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#411-417\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.KeyPressEvent.html#tymethod.response_type\" class=\"fn\">response_type</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u8.html\">u8</a></h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.detail\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#421-427\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.KeyPressEvent.html#tymethod.detail\" class=\"fn\">detail</a>(&amp;self) -&gt; <a class=\"type\" href=\"xcb/x/type.Keycode.html\" title=\"type xcb::x::Keycode\">Keycode</a></h4></section></summary><div class=\"docblock\"><p>The keycode (a number representing a physical key on the keyboard) of the key\nwhich was pressed.</p>\n</div></details><section id=\"method.sequence\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#429-435\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.KeyPressEvent.html#tymethod.sequence\" class=\"fn\">sequence</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u16.html\">u16</a></h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.time\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#438-444\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.KeyPressEvent.html#tymethod.time\" class=\"fn\">time</a>(&amp;self) -&gt; <a class=\"type\" href=\"xcb/x/type.Timestamp.html\" title=\"type xcb::x::Timestamp\">Timestamp</a></h4></section></summary><div class=\"docblock\"><p>Time when the event was generated (in milliseconds).</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.root\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#447-453\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.KeyPressEvent.html#tymethod.root\" class=\"fn\">root</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a></h4></section></summary><div class=\"docblock\"><p>The root window of <code>child</code>.</p>\n</div></details><section id=\"method.event\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#455-461\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.KeyPressEvent.html#tymethod.event\" class=\"fn\">event</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a></h4></section><section id=\"method.child\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#463-469\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.KeyPressEvent.html#tymethod.child\" class=\"fn\">child</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a></h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.root_x\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#473-479\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.KeyPressEvent.html#tymethod.root_x\" class=\"fn\">root_x</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.i16.html\">i16</a></h4></section></summary><div class=\"docblock\"><p>The X coordinate of the pointer relative to the <code>root</code> window at the time of\nthe event.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.root_y\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#483-489\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.KeyPressEvent.html#tymethod.root_y\" class=\"fn\">root_y</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.i16.html\">i16</a></h4></section></summary><div class=\"docblock\"><p>The Y coordinate of the pointer relative to the <code>root</code> window at the time of\nthe event.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.event_x\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#493-499\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.KeyPressEvent.html#tymethod.event_x\" class=\"fn\">event_x</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.i16.html\">i16</a></h4></section></summary><div class=\"docblock\"><p>If <code>same_screen</code> is true, this is the X coordinate relative to the <code>event</code>\nwindow’s origin. Otherwise, <code>event_x</code> will be set to zero.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.event_y\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#503-509\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.KeyPressEvent.html#tymethod.event_y\" class=\"fn\">event_y</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.i16.html\">i16</a></h4></section></summary><div class=\"docblock\"><p>If <code>same_screen</code> is true, this is the Y coordinate relative to the <code>event</code>\nwindow’s origin. Otherwise, <code>event_y</code> will be set to zero.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.state\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#513-520\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.KeyPressEvent.html#tymethod.state\" class=\"fn\">state</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/x/struct.KeyButMask.html\" title=\"struct xcb::x::KeyButMask\">KeyButMask</a></h4></section></summary><div class=\"docblock\"><p>The logical state of the pointer buttons and modifier keys just prior to the\nevent.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.same_screen\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#523-526\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.KeyPressEvent.html#tymethod.same_screen\" class=\"fn\">same_screen</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>Whether the <code>event</code> window is on the same screen as the <code>root</code> window.</p>\n</div></details></div></details>",0,"xcb::xproto::KeyReleaseEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Raw%3Cxcb_generic_event_t%3E-for-KeyPressEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#352-360\">source</a><a href=\"#impl-Raw%3Cxcb_generic_event_t%3E-for-KeyPressEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"xcb/trait.Raw.html\" title=\"trait xcb::Raw\">Raw</a>&lt;<a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_event_t.html\" title=\"struct xcb::ffi::xcb_generic_event_t\">xcb_generic_event_t</a>&gt; for <a class=\"struct\" href=\"xcb/x/struct.KeyPressEvent.html\" title=\"struct xcb::x::KeyPressEvent\">KeyPressEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#353-355\">source</a><a href=\"#method.from_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">unsafe fn <a href=\"xcb/trait.Raw.html#tymethod.from_raw\" class=\"fn\">from_raw</a>(raw: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.pointer.html\">*mut </a><a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_event_t.html\" title=\"struct xcb::ffi::xcb_generic_event_t\">xcb_generic_event_t</a>) -&gt; Self</h4></section></summary><div class='docblock'>Build <code>Self</code> from a raw pointer <a href=\"xcb/trait.Raw.html#tymethod.from_raw\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#357-359\">source</a><a href=\"#method.as_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"xcb/trait.Raw.html#tymethod.as_raw\" class=\"fn\">as_raw</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.pointer.html\">*mut </a><a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_event_t.html\" title=\"struct xcb::ffi::xcb_generic_event_t\">xcb_generic_event_t</a></h4></section></summary><div class='docblock'>Obtain the raw pointer representation</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/base.rs.html#67-71\">source</a><a href=\"#method.into_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"xcb/trait.Raw.html#method.into_raw\" class=\"fn\">into_raw</a>(self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.pointer.html\">*mut T</a></h4></section></summary><div class='docblock'>Convert self into a raw pointer <a href=\"xcb/trait.Raw.html#method.into_raw\">Read more</a></div></details></div></details>","Raw<xcb_generic_event_t>","xcb::xproto::KeyReleaseEvent"],["<section id=\"impl-Send-for-KeyPressEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#588\">source</a><a href=\"#impl-Send-for-KeyPressEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"xcb/x/struct.KeyPressEvent.html\" title=\"struct xcb::x::KeyPressEvent\">KeyPressEvent</a></h3></section>","Send","xcb::xproto::KeyReleaseEvent"],["<section id=\"impl-Sync-for-KeyPressEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#589\">source</a><a href=\"#impl-Sync-for-KeyPressEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"xcb/x/struct.KeyPressEvent.html\" title=\"struct xcb::x::KeyPressEvent\">KeyPressEvent</a></h3></section>","Sync","xcb::xproto::KeyReleaseEvent"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[20428]}