((globalThis) => {
	const { core } = Deno;
	const { ops } = core;

	globalThis.require = (package) => import('file:///' + ops.op_get_package(package));
	globalThis.__dirname = ops.op_dirname();
	globalThis.sleep = (ms) => ops.op_sleep(ms);

	globalThis.core = {
		print: (text) => ops.op_print(text),
		encode: (text) => ops.op_encode(text),
		encode_fast: (text) => ops.op_encode_fast(text),
		escape: (text) => ops.op_escape(text),
		id: {
			secure: (len = 21) => ops.op_id(len),
			basic: (rounds = 4) => [...Array(rounds)].map((i) => Math.round(Date.now() + Math.random() * Date.now()).toString(36)).join(''),
			uuid: () => {
				var lut = [];
				var d0 = (Math.random() * 0xffffffff) | 0;
				var d1 = (Math.random() * 0xffffffff) | 0;
				var d2 = (Math.random() * 0xffffffff) | 0;
				var d3 = (Math.random() * 0xffffffff) | 0;
				for (var i = 0; i < 256; i++) {
					lut[i] = (i < 16 ? '0' : '') + i.toString(16);
				}
				return (
					lut[d0 & 0xff] +
					lut[(d0 >> 8) & 0xff] +
					lut[(d0 >> 16) & 0xff] +
					lut[(d0 >> 24) & 0xff] +
					'-' +
					lut[d1 & 0xff] +
					lut[(d1 >> 8) & 0xff] +
					'-' +
					lut[((d1 >> 16) & 0x0f) | 0x40] +
					lut[(d1 >> 24) & 0xff] +
					'-' +
					lut[(d2 & 0x3f) | 0x80] +
					lut[(d2 >> 8) & 0xff] +
					'-' +
					lut[(d2 >> 16) & 0xff] +
					lut[(d2 >> 24) & 0xff] +
					lut[d3 & 0xff] +
					lut[(d3 >> 8) & 0xff] +
					lut[(d3 >> 16) & 0xff] +
					lut[(d3 >> 24) & 0xff]
				);
			},
		},
	};
})(globalThis);
