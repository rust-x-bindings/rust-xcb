(function() {var type_impls = {
"xcb":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BarrierHitEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4849-5004\">source</a><a href=\"#impl-BarrierHitEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"xcb/xinput/struct.BarrierHitEvent.html\" title=\"struct xcb::xinput::BarrierHitEvent\">BarrierHitEvent</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.response_type\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4854-4860\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.response_type\" class=\"fn\">response_type</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u8.html\">u8</a></h4></section><section id=\"method.extension\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4862-4868\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.extension\" class=\"fn\">extension</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u8.html\">u8</a></h4></section><section id=\"method.sequence\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4870-4876\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.sequence\" class=\"fn\">sequence</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u16.html\">u16</a></h4></section><section id=\"method.length\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4878-4884\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.length\" class=\"fn\">length</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u32.html\">u32</a></h4></section><section id=\"method.event_type\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4886-4892\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.event_type\" class=\"fn\">event_type</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u16.html\">u16</a></h4></section><section id=\"method.device\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4894-4899\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.device\" class=\"fn\">device</a>(&amp;self) -&gt; <a class=\"enum\" href=\"xcb/xinput/enum.Device.html\" title=\"enum xcb::xinput::Device\">Device</a></h4></section><section id=\"method.time\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4901-4907\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.time\" class=\"fn\">time</a>(&amp;self) -&gt; <a class=\"type\" href=\"xcb/x/type.Timestamp.html\" title=\"type xcb::x::Timestamp\">Timestamp</a></h4></section><section id=\"method.eventid\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4909-4915\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.eventid\" class=\"fn\">eventid</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u32.html\">u32</a></h4></section><section id=\"method.root\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4917-4923\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.root\" class=\"fn\">root</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a></h4></section><section id=\"method.event\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4925-4931\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.event\" class=\"fn\">event</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a></h4></section><section id=\"method.barrier\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4933-4939\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.barrier\" class=\"fn\">barrier</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/xfixes/struct.Barrier.html\" title=\"struct xcb::xfixes::Barrier\">Barrier</a></h4></section><section id=\"method.full_sequence\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4941-4947\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.full_sequence\" class=\"fn\">full_sequence</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u32.html\">u32</a></h4></section><section id=\"method.dtime\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4949-4955\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.dtime\" class=\"fn\">dtime</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u32.html\">u32</a></h4></section><section id=\"method.flags\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4957-4964\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.flags\" class=\"fn\">flags</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/xinput/struct.BarrierFlags.html\" title=\"struct xcb::xinput::BarrierFlags\">BarrierFlags</a></h4></section><section id=\"method.source\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4966-4971\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.source\" class=\"fn\">source</a>(&amp;self) -&gt; <a class=\"enum\" href=\"xcb/xinput/enum.Device.html\" title=\"enum xcb::xinput::Device\">Device</a></h4></section><section id=\"method.root_x\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4973-4979\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.root_x\" class=\"fn\">root_x</a>(&amp;self) -&gt; <a class=\"type\" href=\"xcb/xinput/type.Fp1616.html\" title=\"type xcb::xinput::Fp1616\">Fp1616</a></h4></section><section id=\"method.root_y\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4981-4987\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.root_y\" class=\"fn\">root_y</a>(&amp;self) -&gt; <a class=\"type\" href=\"xcb/xinput/type.Fp1616.html\" title=\"type xcb::xinput::Fp1616\">Fp1616</a></h4></section><section id=\"method.dx\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4989-4995\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.dx\" class=\"fn\">dx</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/xinput/struct.Fp3232.html\" title=\"struct xcb::xinput::Fp3232\">Fp3232</a></h4></section><section id=\"method.dy\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4997-5003\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.BarrierHitEvent.html#tymethod.dy\" class=\"fn\">dy</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/xinput/struct.Fp3232.html\" title=\"struct xcb::xinput::Fp3232\">Fp3232</a></h4></section></div></details>",0,"xcb::xinput::BarrierLeaveEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Raw%3Cxcb_ge_generic_event_t%3E-for-BarrierHitEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4833-4841\">source</a><a href=\"#impl-Raw%3Cxcb_ge_generic_event_t%3E-for-BarrierHitEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"xcb/trait.Raw.html\" title=\"trait xcb::Raw\">Raw</a>&lt;<a class=\"struct\" href=\"xcb/ffi/struct.xcb_ge_generic_event_t.html\" title=\"struct xcb::ffi::xcb_ge_generic_event_t\">xcb_ge_generic_event_t</a>&gt; for <a class=\"struct\" href=\"xcb/xinput/struct.BarrierHitEvent.html\" title=\"struct xcb::xinput::BarrierHitEvent\">BarrierHitEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4834-4836\">source</a><a href=\"#method.from_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">unsafe fn <a href=\"xcb/trait.Raw.html#tymethod.from_raw\" class=\"fn\">from_raw</a>(raw: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.pointer.html\">*mut </a><a class=\"struct\" href=\"xcb/ffi/struct.xcb_ge_generic_event_t.html\" title=\"struct xcb::ffi::xcb_ge_generic_event_t\">xcb_ge_generic_event_t</a>) -&gt; Self</h4></section></summary><div class='docblock'>Build <code>Self</code> from a raw pointer <a href=\"xcb/trait.Raw.html#tymethod.from_raw\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4838-4840\">source</a><a href=\"#method.as_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"xcb/trait.Raw.html#tymethod.as_raw\" class=\"fn\">as_raw</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.pointer.html\">*mut </a><a class=\"struct\" href=\"xcb/ffi/struct.xcb_ge_generic_event_t.html\" title=\"struct xcb::ffi::xcb_ge_generic_event_t\">xcb_ge_generic_event_t</a></h4></section></summary><div class='docblock'>Obtain the raw pointer representation</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/base.rs.html#67-71\">source</a><a href=\"#method.into_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"xcb/trait.Raw.html#method.into_raw\" class=\"fn\">into_raw</a>(self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.pointer.html\">*mut T</a></h4></section></summary><div class='docblock'>Convert self into a raw pointer <a href=\"xcb/trait.Raw.html#method.into_raw\">Read more</a></div></details></div></details>","Raw<xcb_ge_generic_event_t>","xcb::xinput::BarrierLeaveEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-GeEvent-for-BarrierHitEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4843-4847\">source</a><a href=\"#impl-GeEvent-for-BarrierHitEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"xcb/trait.GeEvent.html\" title=\"trait xcb::GeEvent\">GeEvent</a> for <a class=\"struct\" href=\"xcb/xinput/struct.BarrierHitEvent.html\" title=\"struct xcb::xinput::BarrierHitEvent\">BarrierHitEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.EXTENSION\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4844\">source</a><a href=\"#associatedconstant.EXTENSION\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"xcb/trait.GeEvent.html#associatedconstant.EXTENSION\" class=\"constant\">EXTENSION</a>: <a class=\"enum\" href=\"xcb/enum.Extension.html\" title=\"enum xcb::Extension\">Extension</a> = ext::Extension::Input</h4></section></summary><div class='docblock'>The extension associated to this event</div></details><details class=\"toggle\" open><summary><section id=\"associatedconstant.NUMBER\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#4846\">source</a><a href=\"#associatedconstant.NUMBER\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"xcb/trait.GeEvent.html#associatedconstant.NUMBER\" class=\"constant\">NUMBER</a>: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u32.html\">u32</a> = 25u32</h4></section></summary><div class='docblock'>The number associated to this event</div></details></div></details>","GeEvent","xcb::xinput::BarrierLeaveEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-BarrierHitEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#5006-5031\">source</a><a href=\"#impl-Debug-for-BarrierHitEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"xcb/xinput/struct.BarrierHitEvent.html\" title=\"struct xcb::xinput::BarrierHitEvent\">BarrierHitEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#5007-5030\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","xcb::xinput::BarrierLeaveEvent"],["<section id=\"impl-Sync-for-BarrierHitEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#5072\">source</a><a href=\"#impl-Sync-for-BarrierHitEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"xcb/xinput/struct.BarrierHitEvent.html\" title=\"struct xcb::xinput::BarrierHitEvent\">BarrierHitEvent</a></h3></section>","Sync","xcb::xinput::BarrierLeaveEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-BarrierHitEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#5063-5069\">source</a><a href=\"#impl-Drop-for-BarrierHitEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"xcb/xinput/struct.BarrierHitEvent.html\" title=\"struct xcb::xinput::BarrierHitEvent\">BarrierHitEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#5064-5068\">source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/1.77.2/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","xcb::xinput::BarrierLeaveEvent"],["<section id=\"impl-Send-for-BarrierHitEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-6f5452bd0e1e77f1/out/xinput.rs.html#5071\">source</a><a href=\"#impl-Send-for-BarrierHitEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"xcb/xinput/struct.BarrierHitEvent.html\" title=\"struct xcb::xinput::BarrierHitEvent\">BarrierHitEvent</a></h3></section>","Send","xcb::xinput::BarrierLeaveEvent"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()