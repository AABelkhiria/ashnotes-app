export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.svg"]),
	mimeTypes: {".svg":"image/svg+xml"},
	_: {
		client: {start:"_app/immutable/entry/start.VSFtxHx8.js",app:"_app/immutable/entry/app.CAKBag5u.js",imports:["_app/immutable/entry/start.VSFtxHx8.js","_app/immutable/chunks/D5uKRjjT.js","_app/immutable/chunks/D_uOQWZ4.js","_app/immutable/entry/app.CAKBag5u.js","_app/immutable/chunks/D_uOQWZ4.js","_app/immutable/chunks/CWj6FrbW.js","_app/immutable/chunks/DJLSrki8.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/2.js')),
			__memo(() => import('./nodes/3.js'))
		],
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			},
			{
				id: "/notes/[...path]",
				pattern: /^\/notes(?:\/(.*))?\/?$/,
				params: [{"name":"path","optional":false,"rest":true,"chained":true}],
				page: { layouts: [0,], errors: [1,], leaf: 3 },
				endpoint: null
			}
		],
		prerendered_routes: new Set([]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
