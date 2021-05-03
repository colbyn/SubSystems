/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + chunkId + ".index.js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"./pkg/index_bg.wasm": function() {
/******/ 			return {
/******/ 				"./index_bg.js": {
/******/ 					"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_string_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_log_386a8115a84a780d": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_log_386a8115a84a780d"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_d14bf16e62c6b3d5": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_new_d14bf16e62c6b3d5"]();
/******/ 					},
/******/ 					"__wbg_set_61642586f7156f4a": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_set_61642586f7156f4a"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_rethrow": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_rethrow"](p0i32);
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"1":["./pkg/index_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"./pkg/index_bg.wasm":"b6de826bf597f845c548"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./main.ts");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./main.ts":
/*!*****************!*\
  !*** ./main.ts ***!
  \*****************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("// Note that a dynamic `import` statement here is required due to\n// webpack/webpack#6615, but in theory `import { greet } from './pkg';`\n// will work here one day as well!\n// const rust = import('./pkg');\nfunction instanceOfElement(object) {\n    if (!instanceOfText(object)) {\n        return 'tag' in object;\n    }\n    else {\n        return false;\n    }\n}\nfunction instanceOfText(object) {\n    return (typeof object === 'string' || object instanceof String);\n}\nfunction tag(tag, attrs, children, apply) {\n    return { tag: tag, attrs: attrs, children: children, apply: apply };\n}\nfunction layout_html(html) {\n    if (instanceOfElement(html)) {\n        var node = document.createElement(html.tag);\n        for (var _i = 0, _a = html.children; _i < _a.length; _i++) {\n            var child = _a[_i];\n            node.appendChild(layout_html(child));\n        }\n        for (var _b = 0, _c = Object.keys(html.attrs); _b < _c.length; _b++) {\n            var key = _c[_b];\n            var value = html.attrs[key];\n            if ((typeof value === 'string') || (value instanceof String)) {\n                node.setAttribute(key, value);\n            }\n            else {\n                node.setAttribute(key, JSON.stringify(value));\n            }\n        }\n        if (html.apply) {\n            html.apply(node);\n        }\n        return node;\n    }\n    else if (instanceOfText(html)) {\n        return document.createTextNode(html);\n    }\n    else {\n        throw \"Invalid Node\";\n    }\n}\nfunction uuid() {\n    return 'UID_xxxxxxxx_xxxx_4xxx_yxxx_xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {\n        var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);\n        return v.toString(16);\n    });\n}\nfunction add_form_entry_function(add_error, code_str, info_str) {\n    var rust = Promise.all(/*! import() */[__webpack_require__.e(0), __webpack_require__.e(1)]).then(__webpack_require__.bind(null, /*! ./pkg/index */ \"./pkg/index.js\"));\n    var uid = uuid();\n    var get_input_value = function (form) {\n        return form.querySelector('input[type=\"text\"]').value;\n    };\n    var node = tag('div', { 'form-wrapper': '', id: uid }, [\n        tag('p', {}, [info_str]),\n        tag('form', { id: uid }, [\n            tag('input', { type: 'text' }, [], function (element) {\n                var input = element;\n                input.value = code_str;\n            }),\n            tag('input', { type: 'submit' }, []),\n        ], function (element) {\n            element.onsubmit = function (event) {\n                rust.then(function (module) {\n                    var input = get_input_value(element);\n                    var result = module.chem_eval(input);\n                    var ast_str = result['ast'];\n                    var value_str = result['value'];\n                    var output = document.querySelector(\"#\" + uid + \" > [output]\");\n                    var new_output = layout_html(tag('table', {}, [\n                        tag('tr', {}, [\n                            tag('td', {}, [\"AST TREE\"]),\n                            tag('td', {}, [\n                                tag('pre', {}, [ast_str])\n                            ]),\n                        ]),\n                        tag('tr', {}, [\n                            tag('td', {}, [\"VALUE\"]),\n                            tag('td', {}, [\n                                tag('code', {}, [value_str])\n                            ]),\n                        ]),\n                    ]));\n                    output.parentElement.replaceChild(new_output, output);\n                }).catch(add_error);\n                console.log('done');\n                return false;\n            };\n        }),\n        tag('div', { output: '' }, []),\n    ]);\n    document.body.appendChild(layout_html(node));\n}\nfunction add_form_entry_reaction(add_error, code_str, info_str) {\n    var rust = Promise.all(/*! import() */[__webpack_require__.e(0), __webpack_require__.e(1)]).then(__webpack_require__.bind(null, /*! ./pkg/index */ \"./pkg/index.js\"));\n    var uid = uuid();\n    var get_input_value = function (form) {\n        return form.querySelector('input[type=\"text\"]').value;\n    };\n    var node = tag('div', { 'form-wrapper': '', id: uid }, [\n        tag('p', {}, [info_str]),\n        tag('form', { id: uid }, [\n            tag('input', { type: 'text' }, [], function (element) {\n                var input = element;\n                input.value = code_str;\n            }),\n            tag('input', { type: 'submit' }, []),\n        ], function (element) {\n            element.onsubmit = function (event) {\n                rust.then(function (module) {\n                    var input = get_input_value(element);\n                    var result = module.balance_reaction(input);\n                    var input_ast = result['input_ast'];\n                    var input_str = result['input_str'];\n                    var output_ast = result['output_ast'];\n                    var output_str = result['output_str'];\n                    var output = document.querySelector(\"#\" + uid + \" > [output]\");\n                    var new_output = layout_html(tag('table', {}, [\n                        tag('tr', {}, [\n                            tag('td', {}, [\"INPUT AST TREE\"]),\n                            tag('td', {}, [\n                                tag('pre', {}, [input_ast])\n                            ]),\n                        ]),\n                        tag('tr', {}, [\n                            tag('td', {}, [\"INPUT VALUE\"]),\n                            tag('td', {}, [\n                                tag('code', {}, [input_str])\n                            ]),\n                        ]),\n                        tag('tr', {}, [\n                            tag('td', {}, [\"OUTPUT AST TREE\"]),\n                            tag('td', {}, [\n                                tag('pre', {}, [output_ast])\n                            ]),\n                        ]),\n                        tag('tr', {}, [\n                            tag('td', {}, [\"OUTPUT VALUE\"]),\n                            tag('td', {}, [\n                                tag('code', {}, [output_str])\n                            ]),\n                        ]),\n                    ]));\n                    output.parentElement.replaceChild(new_output, output);\n                }).catch(add_error);\n                console.log('done');\n                return false;\n            };\n        }),\n        tag('div', { output: '' }, []),\n    ]);\n    document.body.appendChild(layout_html(node));\n}\nwindow.onload = function () {\n    var errors = document.createElement('div');\n    errors.setAttribute('app-errors', '');\n    document.body.appendChild(errors);\n    function add_error(msg) {\n        console.error(msg);\n        var node = document.createElement('p');\n        var stying = \"\\n            color: #131313;\\n            background-color: #ff57574a;\\n            padding: 12px;\\n            font-family: monospace;\\n        \";\n        node.setAttribute('style', stying);\n        node.innerText = msg;\n        node.setAttribute('error', '');\n        errors.appendChild(node);\n    }\n    console.log(\"loaded\");\n    document.body.appendChild(layout_html(tag('h1', {}, [\n        \"Chemical Functions\"\n    ])));\n    add_form_entry_function(add_error, \"mole(energy(photon(wavelength = nm(325))))\", \"energy of one mole of photons given wavelength\");\n    add_form_entry_function(add_error, \"energy(photon(wavelength = nm(325)))\", \"energy of photon given wavelength\");\n    add_form_entry_function(add_error, \"energy(photon(frequency = GHz(275)))\", \"energy of photon given frequency\");\n    add_form_entry_function(add_error, \"wavelength(frequency = MHz(72.5))\", \"wavelength from frequency\");\n    add_form_entry_function(add_error, \"frequency(wavelength = nm(325))\", \"frequency from wavelength\");\n    document.body.appendChild(layout_html(tag('h1', {}, [\n        \"Chemical Reaction\"\n    ])));\n    add_form_entry_reaction(add_error, \"C3H8 + O2 -> CO2 + H2O\", \"balance chemical reactions\");\n    add_form_entry_reaction(add_error, \"Ca(O3H2)2(aq) + HCl(aq) -> CaCl2(aq) + H2O(l)\", \"balance chemical reactions\");\n    add_form_entry_reaction(add_error, \"PCl5 + H2O -> H3PO4 + HCl\", \"balance chemical reactions\");\n    add_form_entry_reaction(add_error, \"Al + O2 -> Al2O3\", \"balance chemical reactions\");\n    add_form_entry_reaction(add_error, \"Na3PO4 + Ba(NO3)2 -> Ba3(PO4)2 + NaNO3\", \"balance chemical reactions\");\n    add_form_entry_reaction(add_error, \"2XY2 + 4Y -> 8X2 + 8YX\", \"balance chemical reactions\");\n};\n\n\n//# sourceURL=webpack:///./main.ts?");

/***/ })

/******/ });