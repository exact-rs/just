((globalThis) => {
	Object.defineProperty(String.prototype, 'parseBytes', {
		value(decimals = 2) {
			if (!+this) return '0B';
			const c = 0 > decimals ? 0 : decimals,
				d = Math.floor(Math.log(this) / Math.log(1024));
			return `${parseFloat((this / Math.pow(1024, d)).toFixed(c))}${['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'][d]}`;
		},
	});

	Object.defineProperty(String.prototype, 'json', {
		value() {
			return JSON.parse(this);
		},
	});

	// Object.defineProperty(String.prototype, 'jsonString', {
	// 	value() {
	// 		return JSON.parse(
	// 			`[${Array.from(this.slice(1, -1).replace(/"(-|)([0-9]+(?:\.[0-9]+)?)"/g, '$1$2'))
	// 				.slice(1, -1)
	// 				.join('')
	// 				.split('}","{')
	// 				.join('},{')}]`
	// 		);
	// 	},
	// });

	Object.defineProperty(String.prototype, 'reverse', {
		value() {
			return this.split('').reverse().join('');
		},
	});

	globalThis.string = {};
})(globalThis);
