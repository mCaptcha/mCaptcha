/******/ (function(modules) { // webpackBootstrap
/******/ 	// The module cache
/******/ 	var installedModules = {};
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
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./templates/index.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./node_modules/css-loader/dist/cjs.js!./node_modules/sass-loader/dist/cjs.js!./templates/auth/forms.scss":
/*!****************************************************************************************************************!*\
  !*** ./node_modules/css-loader/dist/cjs.js!./node_modules/sass-loader/dist/cjs.js!./templates/auth/forms.scss ***!
  \****************************************************************************************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("exports = module.exports = __webpack_require__(/*! ../../node_modules/css-loader/dist/runtime/api.js */ \"./node_modules/css-loader/dist/runtime/api.js\")(false);\n// Module\nexports.push([module.i, \"* {\\n  padding: 0;\\n  margin: 0; }\\n\\n.form__logo {\\n  width: 110px;\\n  padding-top: 50px;\\n  display: block;\\n  margin: auto;\\n  position: relative;\\n  top: 20%;\\n  transform: translate(0%, -40.9%); }\\n\\n.form__brand {\\n  padding: 10px 0;\\n  text-align: center;\\n  position: relative;\\n  top: 20%;\\n  transform: translate(0%, -90.9%); }\\n\\n.form-container {\\n  max-width: 40%;\\n  min-width: 20%;\\n  position: absolute;\\n  top: 50%;\\n  left: 50%;\\n  transform: translate(-50%, -49.9%);\\n  box-sizing: border-box;\\n  margin: auto;\\n  padding: 20px 0; }\\n\\n.form__box {\\n  border: 1px solid #eaecef;\\n  background-color: #f6f8fa;\\n  border-radius: 5px;\\n  padding: 20px 0; }\\n\\n.form__in-group {\\n  display: block;\\n  position: relative;\\n  margin: auto;\\n  max-width: 80%;\\n  padding: 10px 0px;\\n  box-sizing: content-box;\\n  align-items: center;\\n  align-content: center; }\\n\\n.form__in-field {\\n  display: block;\\n  box-sizing: border-box;\\n  margin: 10px 0;\\n  padding: 10px 0;\\n  width: 100%; }\\n\\n.form__in-field--warn {\\n  border: solid 1px red; }\\n\\n.form__in-field--success {\\n  border: solid 1px #2ea44f; }\\n\\n.form__pw-recovery {\\n  text-decoration: none;\\n  color: #0366d6;\\n  font-size: 0.8rem; }\\n\\n.form__submit-button {\\n  display: block;\\n  border: 1px solid skyblue;\\n  background: #2ea44f;\\n  color: white;\\n  height: 40px;\\n  border-radius: 5px;\\n  width: 80%;\\n  margin: auto; }\\n\\n.form__secondary-action {\\n  display: block;\\n  margin-top: 10px; }\\n\\n.form__secondary-action__banner {\\n  display: block;\\n  margin: auto;\\n  max-width: 80%;\\n  text-align: center; }\\n\\n.form__secondary-action__link {\\n  text-decoration: none;\\n  color: #0366d6; }\\n\", \"\"]);\n\n\n\n//# sourceURL=webpack:///./templates/auth/forms.scss?./node_modules/css-loader/dist/cjs.js!./node_modules/sass-loader/dist/cjs.js");

/***/ }),

/***/ "./node_modules/css-loader/dist/runtime/api.js":
/*!*****************************************************!*\
  !*** ./node_modules/css-loader/dist/runtime/api.js ***!
  \*****************************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

"use strict";
eval("\n\n/*\n  MIT License http://www.opensource.org/licenses/mit-license.php\n  Author Tobias Koppers @sokra\n*/\n// css base code, injected by the css-loader\nmodule.exports = function (useSourceMap) {\n  var list = []; // return the list of modules as css string\n\n  list.toString = function toString() {\n    return this.map(function (item) {\n      var content = cssWithMappingToString(item, useSourceMap);\n\n      if (item[2]) {\n        return '@media ' + item[2] + '{' + content + '}';\n      } else {\n        return content;\n      }\n    }).join('');\n  }; // import a list of modules into the list\n\n\n  list.i = function (modules, mediaQuery) {\n    if (typeof modules === 'string') {\n      modules = [[null, modules, '']];\n    }\n\n    var alreadyImportedModules = {};\n\n    for (var i = 0; i < this.length; i++) {\n      var id = this[i][0];\n\n      if (id != null) {\n        alreadyImportedModules[id] = true;\n      }\n    }\n\n    for (i = 0; i < modules.length; i++) {\n      var item = modules[i]; // skip already imported module\n      // this implementation is not 100% perfect for weird media query combinations\n      // when a module is imported multiple times with different media queries.\n      // I hope this will never occur (Hey this way we have smaller bundles)\n\n      if (item[0] == null || !alreadyImportedModules[item[0]]) {\n        if (mediaQuery && !item[2]) {\n          item[2] = mediaQuery;\n        } else if (mediaQuery) {\n          item[2] = '(' + item[2] + ') and (' + mediaQuery + ')';\n        }\n\n        list.push(item);\n      }\n    }\n  };\n\n  return list;\n};\n\nfunction cssWithMappingToString(item, useSourceMap) {\n  var content = item[1] || '';\n  var cssMapping = item[3];\n\n  if (!cssMapping) {\n    return content;\n  }\n\n  if (useSourceMap && typeof btoa === 'function') {\n    var sourceMapping = toComment(cssMapping);\n    var sourceURLs = cssMapping.sources.map(function (source) {\n      return '/*# sourceURL=' + cssMapping.sourceRoot + source + ' */';\n    });\n    return [content].concat(sourceURLs).concat([sourceMapping]).join('\\n');\n  }\n\n  return [content].join('\\n');\n} // Adapted from convert-source-map (MIT)\n\n\nfunction toComment(sourceMap) {\n  // eslint-disable-next-line no-undef\n  var base64 = btoa(unescape(encodeURIComponent(JSON.stringify(sourceMap))));\n  var data = 'sourceMappingURL=data:application/json;charset=utf-8;base64,' + base64;\n  return '/*# ' + data + ' */';\n}\n\n//# sourceURL=webpack:///./node_modules/css-loader/dist/runtime/api.js?");

/***/ }),

/***/ "./node_modules/style-loader/lib/addStyles.js":
/*!****************************************************!*\
  !*** ./node_modules/style-loader/lib/addStyles.js ***!
  \****************************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("/*\n\tMIT License http://www.opensource.org/licenses/mit-license.php\n\tAuthor Tobias Koppers @sokra\n*/\n\nvar stylesInDom = {};\n\nvar\tmemoize = function (fn) {\n\tvar memo;\n\n\treturn function () {\n\t\tif (typeof memo === \"undefined\") memo = fn.apply(this, arguments);\n\t\treturn memo;\n\t};\n};\n\nvar isOldIE = memoize(function () {\n\t// Test for IE <= 9 as proposed by Browserhacks\n\t// @see http://browserhacks.com/#hack-e71d8692f65334173fee715c222cb805\n\t// Tests for existence of standard globals is to allow style-loader\n\t// to operate correctly into non-standard environments\n\t// @see https://github.com/webpack-contrib/style-loader/issues/177\n\treturn window && document && document.all && !window.atob;\n});\n\nvar getTarget = function (target, parent) {\n  if (parent){\n    return parent.querySelector(target);\n  }\n  return document.querySelector(target);\n};\n\nvar getElement = (function (fn) {\n\tvar memo = {};\n\n\treturn function(target, parent) {\n                // If passing function in options, then use it for resolve \"head\" element.\n                // Useful for Shadow Root style i.e\n                // {\n                //   insertInto: function () { return document.querySelector(\"#foo\").shadowRoot }\n                // }\n                if (typeof target === 'function') {\n                        return target();\n                }\n                if (typeof memo[target] === \"undefined\") {\n\t\t\tvar styleTarget = getTarget.call(this, target, parent);\n\t\t\t// Special case to return head of iframe instead of iframe itself\n\t\t\tif (window.HTMLIFrameElement && styleTarget instanceof window.HTMLIFrameElement) {\n\t\t\t\ttry {\n\t\t\t\t\t// This will throw an exception if access to iframe is blocked\n\t\t\t\t\t// due to cross-origin restrictions\n\t\t\t\t\tstyleTarget = styleTarget.contentDocument.head;\n\t\t\t\t} catch(e) {\n\t\t\t\t\tstyleTarget = null;\n\t\t\t\t}\n\t\t\t}\n\t\t\tmemo[target] = styleTarget;\n\t\t}\n\t\treturn memo[target]\n\t};\n})();\n\nvar singleton = null;\nvar\tsingletonCounter = 0;\nvar\tstylesInsertedAtTop = [];\n\nvar\tfixUrls = __webpack_require__(/*! ./urls */ \"./node_modules/style-loader/lib/urls.js\");\n\nmodule.exports = function(list, options) {\n\tif (typeof DEBUG !== \"undefined\" && DEBUG) {\n\t\tif (typeof document !== \"object\") throw new Error(\"The style-loader cannot be used in a non-browser environment\");\n\t}\n\n\toptions = options || {};\n\n\toptions.attrs = typeof options.attrs === \"object\" ? options.attrs : {};\n\n\t// Force single-tag solution on IE6-9, which has a hard limit on the # of <style>\n\t// tags it will allow on a page\n\tif (!options.singleton && typeof options.singleton !== \"boolean\") options.singleton = isOldIE();\n\n\t// By default, add <style> tags to the <head> element\n        if (!options.insertInto) options.insertInto = \"head\";\n\n\t// By default, add <style> tags to the bottom of the target\n\tif (!options.insertAt) options.insertAt = \"bottom\";\n\n\tvar styles = listToStyles(list, options);\n\n\taddStylesToDom(styles, options);\n\n\treturn function update (newList) {\n\t\tvar mayRemove = [];\n\n\t\tfor (var i = 0; i < styles.length; i++) {\n\t\t\tvar item = styles[i];\n\t\t\tvar domStyle = stylesInDom[item.id];\n\n\t\t\tdomStyle.refs--;\n\t\t\tmayRemove.push(domStyle);\n\t\t}\n\n\t\tif(newList) {\n\t\t\tvar newStyles = listToStyles(newList, options);\n\t\t\taddStylesToDom(newStyles, options);\n\t\t}\n\n\t\tfor (var i = 0; i < mayRemove.length; i++) {\n\t\t\tvar domStyle = mayRemove[i];\n\n\t\t\tif(domStyle.refs === 0) {\n\t\t\t\tfor (var j = 0; j < domStyle.parts.length; j++) domStyle.parts[j]();\n\n\t\t\t\tdelete stylesInDom[domStyle.id];\n\t\t\t}\n\t\t}\n\t};\n};\n\nfunction addStylesToDom (styles, options) {\n\tfor (var i = 0; i < styles.length; i++) {\n\t\tvar item = styles[i];\n\t\tvar domStyle = stylesInDom[item.id];\n\n\t\tif(domStyle) {\n\t\t\tdomStyle.refs++;\n\n\t\t\tfor(var j = 0; j < domStyle.parts.length; j++) {\n\t\t\t\tdomStyle.parts[j](item.parts[j]);\n\t\t\t}\n\n\t\t\tfor(; j < item.parts.length; j++) {\n\t\t\t\tdomStyle.parts.push(addStyle(item.parts[j], options));\n\t\t\t}\n\t\t} else {\n\t\t\tvar parts = [];\n\n\t\t\tfor(var j = 0; j < item.parts.length; j++) {\n\t\t\t\tparts.push(addStyle(item.parts[j], options));\n\t\t\t}\n\n\t\t\tstylesInDom[item.id] = {id: item.id, refs: 1, parts: parts};\n\t\t}\n\t}\n}\n\nfunction listToStyles (list, options) {\n\tvar styles = [];\n\tvar newStyles = {};\n\n\tfor (var i = 0; i < list.length; i++) {\n\t\tvar item = list[i];\n\t\tvar id = options.base ? item[0] + options.base : item[0];\n\t\tvar css = item[1];\n\t\tvar media = item[2];\n\t\tvar sourceMap = item[3];\n\t\tvar part = {css: css, media: media, sourceMap: sourceMap};\n\n\t\tif(!newStyles[id]) styles.push(newStyles[id] = {id: id, parts: [part]});\n\t\telse newStyles[id].parts.push(part);\n\t}\n\n\treturn styles;\n}\n\nfunction insertStyleElement (options, style) {\n\tvar target = getElement(options.insertInto)\n\n\tif (!target) {\n\t\tthrow new Error(\"Couldn't find a style target. This probably means that the value for the 'insertInto' parameter is invalid.\");\n\t}\n\n\tvar lastStyleElementInsertedAtTop = stylesInsertedAtTop[stylesInsertedAtTop.length - 1];\n\n\tif (options.insertAt === \"top\") {\n\t\tif (!lastStyleElementInsertedAtTop) {\n\t\t\ttarget.insertBefore(style, target.firstChild);\n\t\t} else if (lastStyleElementInsertedAtTop.nextSibling) {\n\t\t\ttarget.insertBefore(style, lastStyleElementInsertedAtTop.nextSibling);\n\t\t} else {\n\t\t\ttarget.appendChild(style);\n\t\t}\n\t\tstylesInsertedAtTop.push(style);\n\t} else if (options.insertAt === \"bottom\") {\n\t\ttarget.appendChild(style);\n\t} else if (typeof options.insertAt === \"object\" && options.insertAt.before) {\n\t\tvar nextSibling = getElement(options.insertAt.before, target);\n\t\ttarget.insertBefore(style, nextSibling);\n\t} else {\n\t\tthrow new Error(\"[Style Loader]\\n\\n Invalid value for parameter 'insertAt' ('options.insertAt') found.\\n Must be 'top', 'bottom', or Object.\\n (https://github.com/webpack-contrib/style-loader#insertat)\\n\");\n\t}\n}\n\nfunction removeStyleElement (style) {\n\tif (style.parentNode === null) return false;\n\tstyle.parentNode.removeChild(style);\n\n\tvar idx = stylesInsertedAtTop.indexOf(style);\n\tif(idx >= 0) {\n\t\tstylesInsertedAtTop.splice(idx, 1);\n\t}\n}\n\nfunction createStyleElement (options) {\n\tvar style = document.createElement(\"style\");\n\n\tif(options.attrs.type === undefined) {\n\t\toptions.attrs.type = \"text/css\";\n\t}\n\n\tif(options.attrs.nonce === undefined) {\n\t\tvar nonce = getNonce();\n\t\tif (nonce) {\n\t\t\toptions.attrs.nonce = nonce;\n\t\t}\n\t}\n\n\taddAttrs(style, options.attrs);\n\tinsertStyleElement(options, style);\n\n\treturn style;\n}\n\nfunction createLinkElement (options) {\n\tvar link = document.createElement(\"link\");\n\n\tif(options.attrs.type === undefined) {\n\t\toptions.attrs.type = \"text/css\";\n\t}\n\toptions.attrs.rel = \"stylesheet\";\n\n\taddAttrs(link, options.attrs);\n\tinsertStyleElement(options, link);\n\n\treturn link;\n}\n\nfunction addAttrs (el, attrs) {\n\tObject.keys(attrs).forEach(function (key) {\n\t\tel.setAttribute(key, attrs[key]);\n\t});\n}\n\nfunction getNonce() {\n\tif (false) {}\n\n\treturn __webpack_require__.nc;\n}\n\nfunction addStyle (obj, options) {\n\tvar style, update, remove, result;\n\n\t// If a transform function was defined, run it on the css\n\tif (options.transform && obj.css) {\n\t    result = typeof options.transform === 'function'\n\t\t ? options.transform(obj.css) \n\t\t : options.transform.default(obj.css);\n\n\t    if (result) {\n\t    \t// If transform returns a value, use that instead of the original css.\n\t    \t// This allows running runtime transformations on the css.\n\t    \tobj.css = result;\n\t    } else {\n\t    \t// If the transform function returns a falsy value, don't add this css.\n\t    \t// This allows conditional loading of css\n\t    \treturn function() {\n\t    \t\t// noop\n\t    \t};\n\t    }\n\t}\n\n\tif (options.singleton) {\n\t\tvar styleIndex = singletonCounter++;\n\n\t\tstyle = singleton || (singleton = createStyleElement(options));\n\n\t\tupdate = applyToSingletonTag.bind(null, style, styleIndex, false);\n\t\tremove = applyToSingletonTag.bind(null, style, styleIndex, true);\n\n\t} else if (\n\t\tobj.sourceMap &&\n\t\ttypeof URL === \"function\" &&\n\t\ttypeof URL.createObjectURL === \"function\" &&\n\t\ttypeof URL.revokeObjectURL === \"function\" &&\n\t\ttypeof Blob === \"function\" &&\n\t\ttypeof btoa === \"function\"\n\t) {\n\t\tstyle = createLinkElement(options);\n\t\tupdate = updateLink.bind(null, style, options);\n\t\tremove = function () {\n\t\t\tremoveStyleElement(style);\n\n\t\t\tif(style.href) URL.revokeObjectURL(style.href);\n\t\t};\n\t} else {\n\t\tstyle = createStyleElement(options);\n\t\tupdate = applyToTag.bind(null, style);\n\t\tremove = function () {\n\t\t\tremoveStyleElement(style);\n\t\t};\n\t}\n\n\tupdate(obj);\n\n\treturn function updateStyle (newObj) {\n\t\tif (newObj) {\n\t\t\tif (\n\t\t\t\tnewObj.css === obj.css &&\n\t\t\t\tnewObj.media === obj.media &&\n\t\t\t\tnewObj.sourceMap === obj.sourceMap\n\t\t\t) {\n\t\t\t\treturn;\n\t\t\t}\n\n\t\t\tupdate(obj = newObj);\n\t\t} else {\n\t\t\tremove();\n\t\t}\n\t};\n}\n\nvar replaceText = (function () {\n\tvar textStore = [];\n\n\treturn function (index, replacement) {\n\t\ttextStore[index] = replacement;\n\n\t\treturn textStore.filter(Boolean).join('\\n');\n\t};\n})();\n\nfunction applyToSingletonTag (style, index, remove, obj) {\n\tvar css = remove ? \"\" : obj.css;\n\n\tif (style.styleSheet) {\n\t\tstyle.styleSheet.cssText = replaceText(index, css);\n\t} else {\n\t\tvar cssNode = document.createTextNode(css);\n\t\tvar childNodes = style.childNodes;\n\n\t\tif (childNodes[index]) style.removeChild(childNodes[index]);\n\n\t\tif (childNodes.length) {\n\t\t\tstyle.insertBefore(cssNode, childNodes[index]);\n\t\t} else {\n\t\t\tstyle.appendChild(cssNode);\n\t\t}\n\t}\n}\n\nfunction applyToTag (style, obj) {\n\tvar css = obj.css;\n\tvar media = obj.media;\n\n\tif(media) {\n\t\tstyle.setAttribute(\"media\", media)\n\t}\n\n\tif(style.styleSheet) {\n\t\tstyle.styleSheet.cssText = css;\n\t} else {\n\t\twhile(style.firstChild) {\n\t\t\tstyle.removeChild(style.firstChild);\n\t\t}\n\n\t\tstyle.appendChild(document.createTextNode(css));\n\t}\n}\n\nfunction updateLink (link, options, obj) {\n\tvar css = obj.css;\n\tvar sourceMap = obj.sourceMap;\n\n\t/*\n\t\tIf convertToAbsoluteUrls isn't defined, but sourcemaps are enabled\n\t\tand there is no publicPath defined then lets turn convertToAbsoluteUrls\n\t\ton by default.  Otherwise default to the convertToAbsoluteUrls option\n\t\tdirectly\n\t*/\n\tvar autoFixUrls = options.convertToAbsoluteUrls === undefined && sourceMap;\n\n\tif (options.convertToAbsoluteUrls || autoFixUrls) {\n\t\tcss = fixUrls(css);\n\t}\n\n\tif (sourceMap) {\n\t\t// http://stackoverflow.com/a/26603875\n\t\tcss += \"\\n/*# sourceMappingURL=data:application/json;base64,\" + btoa(unescape(encodeURIComponent(JSON.stringify(sourceMap)))) + \" */\";\n\t}\n\n\tvar blob = new Blob([css], { type: \"text/css\" });\n\n\tvar oldSrc = link.href;\n\n\tlink.href = URL.createObjectURL(blob);\n\n\tif(oldSrc) URL.revokeObjectURL(oldSrc);\n}\n\n\n//# sourceURL=webpack:///./node_modules/style-loader/lib/addStyles.js?");

/***/ }),

/***/ "./node_modules/style-loader/lib/urls.js":
/*!***********************************************!*\
  !*** ./node_modules/style-loader/lib/urls.js ***!
  \***********************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("\n/**\n * When source maps are enabled, `style-loader` uses a link element with a data-uri to\n * embed the css on the page. This breaks all relative urls because now they are relative to a\n * bundle instead of the current page.\n *\n * One solution is to only use full urls, but that may be impossible.\n *\n * Instead, this function \"fixes\" the relative urls to be absolute according to the current page location.\n *\n * A rudimentary test suite is located at `test/fixUrls.js` and can be run via the `npm test` command.\n *\n */\n\nmodule.exports = function (css) {\n  // get current location\n  var location = typeof window !== \"undefined\" && window.location;\n\n  if (!location) {\n    throw new Error(\"fixUrls requires window.location\");\n  }\n\n\t// blank or null?\n\tif (!css || typeof css !== \"string\") {\n\t  return css;\n  }\n\n  var baseUrl = location.protocol + \"//\" + location.host;\n  var currentDir = baseUrl + location.pathname.replace(/\\/[^\\/]*$/, \"/\");\n\n\t// convert each url(...)\n\t/*\n\tThis regular expression is just a way to recursively match brackets within\n\ta string.\n\n\t /url\\s*\\(  = Match on the word \"url\" with any whitespace after it and then a parens\n\t   (  = Start a capturing group\n\t     (?:  = Start a non-capturing group\n\t         [^)(]  = Match anything that isn't a parentheses\n\t         |  = OR\n\t         \\(  = Match a start parentheses\n\t             (?:  = Start another non-capturing groups\n\t                 [^)(]+  = Match anything that isn't a parentheses\n\t                 |  = OR\n\t                 \\(  = Match a start parentheses\n\t                     [^)(]*  = Match anything that isn't a parentheses\n\t                 \\)  = Match a end parentheses\n\t             )  = End Group\n              *\\) = Match anything and then a close parens\n          )  = Close non-capturing group\n          *  = Match anything\n       )  = Close capturing group\n\t \\)  = Match a close parens\n\n\t /gi  = Get all matches, not the first.  Be case insensitive.\n\t */\n\tvar fixedCss = css.replace(/url\\s*\\(((?:[^)(]|\\((?:[^)(]+|\\([^)(]*\\))*\\))*)\\)/gi, function(fullMatch, origUrl) {\n\t\t// strip quotes (if they exist)\n\t\tvar unquotedOrigUrl = origUrl\n\t\t\t.trim()\n\t\t\t.replace(/^\"(.*)\"$/, function(o, $1){ return $1; })\n\t\t\t.replace(/^'(.*)'$/, function(o, $1){ return $1; });\n\n\t\t// already a full url? no change\n\t\tif (/^(#|data:|http:\\/\\/|https:\\/\\/|file:\\/\\/\\/|\\s*$)/i.test(unquotedOrigUrl)) {\n\t\t  return fullMatch;\n\t\t}\n\n\t\t// convert the url to a full url\n\t\tvar newUrl;\n\n\t\tif (unquotedOrigUrl.indexOf(\"//\") === 0) {\n\t\t  \t//TODO: should we add protocol?\n\t\t\tnewUrl = unquotedOrigUrl;\n\t\t} else if (unquotedOrigUrl.indexOf(\"/\") === 0) {\n\t\t\t// path should be relative to the base url\n\t\t\tnewUrl = baseUrl + unquotedOrigUrl; // already starts with '/'\n\t\t} else {\n\t\t\t// path should be relative to current directory\n\t\t\tnewUrl = currentDir + unquotedOrigUrl.replace(/^\\.\\//, \"\"); // Strip leading './'\n\t\t}\n\n\t\t// send back the fixed url(...)\n\t\treturn \"url(\" + JSON.stringify(newUrl) + \")\";\n\t});\n\n\t// send back the fixed css\n\treturn fixedCss;\n};\n\n\n//# sourceURL=webpack:///./node_modules/style-loader/lib/urls.js?");

/***/ }),

/***/ "./templates/api/v1/routes.js":
/*!************************************!*\
  !*** ./templates/api/v1/routes.js ***!
  \************************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\nconst ROUTES = {\n  registerUser: '/api/v1/signup',\n\n  loginUser: '/api/v1/signin',\n\n  signoutUser: '/api/v1/signout',\n\n  deleteAccount: '/api/v1/account/delete',\n\n  usernameExists: '/api/v1/account/username/exists',\n\n  emailExists: '/api/v1/account/email/exists',\n\n  healthCheck: '/api/v1/meta/health',\n\n  buildDetails: '/api/v1/meta/build',\n\n  addDomain: '/api/v1/mcaptcha/domain/add',\n\n  challengeDomain: '/api/v1/mcaptcha/domain/domain/verify/challenge/get',\n\n  proveDomain: '/api/v1/mcaptcha/domain/domain/verify/challenge/prove',\n\n  deleteDomain: '/api/v1/mcaptcha/domain/delete',\n\n  addToken: '/api/v1/mcaptcha/domain/token/add',\n\n  updateTokenKey: '/api/v1/mcaptcha/domain/token/update',\n\n  getTokenKey: '/api/v1/mcaptcha/domain/token/get',\n\n  deleteToken: '/api/v1/mcaptcha/domain/token/delete',\n\n  addTokenLevels: '/api/v1/mcaptcha/domain/token/levels/add',\n\n  updateTokenLevels: '/api/v1/mcaptcha/domain/token/levels/update',\n\n  deleteTokenLevels: '/api/v1/mcaptcha/domain/token/levels/delete',\n\n  getTokenLevels: '/api/v1/mcaptcha/domain/token/levels/get',\n\n  getTokenDuration: '/api/v1/mcaptcha/domain/token/token/get',\n\n  updateTokenDuration: '/api/v1/mcaptcha/domain/token/token/update',\n};\n\n/* harmony default export */ __webpack_exports__[\"default\"] = (ROUTES);\n\n\n//# sourceURL=webpack:///./templates/api/v1/routes.js?");

/***/ }),

/***/ "./templates/auth/forms.scss":
/*!***********************************!*\
  !*** ./templates/auth/forms.scss ***!
  \***********************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("\nvar content = __webpack_require__(/*! !../../node_modules/css-loader/dist/cjs.js!../../node_modules/sass-loader/dist/cjs.js!./forms.scss */ \"./node_modules/css-loader/dist/cjs.js!./node_modules/sass-loader/dist/cjs.js!./templates/auth/forms.scss\");\n\nif(typeof content === 'string') content = [[module.i, content, '']];\n\nvar transform;\nvar insertInto;\n\n\n\nvar options = {\"hmr\":true}\n\noptions.transform = transform\noptions.insertInto = undefined;\n\nvar update = __webpack_require__(/*! ../../node_modules/style-loader/lib/addStyles.js */ \"./node_modules/style-loader/lib/addStyles.js\")(content, options);\n\nif(content.locals) module.exports = content.locals;\n\nif(false) {}\n\n//# sourceURL=webpack:///./templates/auth/forms.scss?");

/***/ }),

/***/ "./templates/auth/login/index.js":
/*!***************************************!*\
  !*** ./templates/auth/login/index.js ***!
  \***************************************/
/*! exports provided: index */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"index\", function() { return index; });\n/* harmony import */ var _api_v1_routes__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../../api/v1/routes */ \"./templates/api/v1/routes.js\");\n/* harmony import */ var _utils_genJsonPayload__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../../utils/genJsonPayload */ \"./templates/utils/genJsonPayload.js\");\n/* harmony import */ var _forms_scss__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ../forms.scss */ \"./templates/auth/forms.scss\");\n/* harmony import */ var _forms_scss__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(_forms_scss__WEBPACK_IMPORTED_MODULE_2__);\n\n\n\n\n\n\n\nconst login = e => {\n  e.preventDefault();\n  let username = document.getElementById('username').value;\n  Object(_utils_genJsonPayload__WEBPACK_IMPORTED_MODULE_1__[\"default\"])(e, username, 'username');\n\n  let password = document.getElementById('password').value;\n  let payload = {\n    username,\n    password,\n  };\n\n  fetch(_api_v1_routes__WEBPACK_IMPORTED_MODULE_0__[\"default\"].loginUser, Object(_utils_genJsonPayload__WEBPACK_IMPORTED_MODULE_1__[\"default\"])(payload)).then(res => {\n    if (res.ok) {\n      alert('success');\n    } else {\n      res.json().then(err => alert(`error: ${err.error}`));\n    }\n  });\n};\n\nconst index = () => {\n  let form = document.getElementById('form');\n  form.addEventListener('submit', login, true);\n};\n\n\n//# sourceURL=webpack:///./templates/auth/login/index.js?");

/***/ }),

/***/ "./templates/auth/register/emailExists.js":
/*!************************************************!*\
  !*** ./templates/auth/register/emailExists.js ***!
  \************************************************/
/*! exports provided: checkEmailExists */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"checkEmailExists\", function() { return checkEmailExists; });\n/* harmony import */ var _api_v1_routes__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../../api/v1/routes */ \"./templates/api/v1/routes.js\");\n/* harmony import */ var _utils_genJsonPayload__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../../utils/genJsonPayload */ \"./templates/utils/genJsonPayload.js\");\n\n\n\n\nconst checkEmailExists = async () => {\n  let email = document.getElementById('email');\n  let val = email.value;\n  let payload = {\n    val,\n  };\n\n  //  return fetch(ROUTES.emailExists, genJsonPayload(payload)).then(res => {\n  //    if (res.ok) {\n  //      res.json().then(data => {\n  //        if (data.exists) {\n  //          console.log(email.className);\n  //          email.className += ' form__in-field--warn';\n  //          alert('Email taken');\n  //        }\n  //\n  //        return data.exists;\n  //      });\n  //    } else {\n  //      res.json().then(err => alert(`error: ${err.error}`));\n  //    }\n  //  });\n  //\n\n  let res = await fetch(_api_v1_routes__WEBPACK_IMPORTED_MODULE_0__[\"default\"].emailExists, Object(_utils_genJsonPayload__WEBPACK_IMPORTED_MODULE_1__[\"default\"])(payload));\n  if (res.ok) {\n    let data = await res.json();\n    if (data.exists) {\n      email.className += ' form__in-field--warn';\n      alert('Email taken');\n    }\n    return data.exists;\n  } else {\n    let err = await res.json();\n    alert(`error: ${err.error}`);\n  }\n};\n\n\n\n\n//# sourceURL=webpack:///./templates/auth/register/emailExists.js?");

/***/ }),

/***/ "./templates/auth/register/index.js":
/*!******************************************!*\
  !*** ./templates/auth/register/index.js ***!
  \******************************************/
/*! exports provided: index */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"index\", function() { return index; });\n/* harmony import */ var _api_v1_routes__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../../api/v1/routes */ \"./templates/api/v1/routes.js\");\n/* harmony import */ var _utils_genJsonPayload__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../../utils/genJsonPayload */ \"./templates/utils/genJsonPayload.js\");\n/* harmony import */ var _userExists__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./userExists */ \"./templates/auth/register/userExists.js\");\n/* harmony import */ var _emailExists__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./emailExists */ \"./templates/auth/register/emailExists.js\");\n/* harmony import */ var _forms_scss__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ../forms.scss */ \"./templates/auth/forms.scss\");\n/* harmony import */ var _forms_scss__WEBPACK_IMPORTED_MODULE_4___default = /*#__PURE__*/__webpack_require__.n(_forms_scss__WEBPACK_IMPORTED_MODULE_4__);\n\n\n\n\n\n\n\n\n\n\nconst registerUser = async e => {\n  e.preventDefault();\n\n  let username = document.getElementById('username').value;\n  Object(_utils_genJsonPayload__WEBPACK_IMPORTED_MODULE_1__[\"default\"])(e, username, 'username');\n\n  let password = document.getElementById('password').value;\n  let passwordCheck = document.getElementById('password-check').value;\n  if (password != passwordCheck) {\n    return alert(\"passwords don't match, check again!\");\n  }\n\n  let exists = await Object(_userExists__WEBPACK_IMPORTED_MODULE_2__[\"default\"])();\n  if (exists) {\n    return;\n  }\n\n  let email = document.getElementById('email').value;\n  if (!email.replace(/\\s/g, '').length) {\n    email = null;\n  } else {\n    exists = await Object(_emailExists__WEBPACK_IMPORTED_MODULE_3__[\"checkEmailExists\"])();\n    if (exists) {\n      return;\n    }\n  }\n\n  let payload = {\n    username,\n    password,\n    email,\n  };\n\n  let res = await fetch(_api_v1_routes__WEBPACK_IMPORTED_MODULE_0__[\"default\"].registerUser, Object(_utils_genJsonPayload__WEBPACK_IMPORTED_MODULE_1__[\"default\"])(payload));\n  if (res.ok) {\n    alert('success');\n  } else {\n    let err = await res.json();\n    alert(`error: ${err.error}`);\n  }\n};\n\nconst index = () => {\n  let form = document.getElementById('form');\n  form.addEventListener('submit', registerUser, true);\n\n  let username = document.getElementById('username');\n  username.addEventListener('input', _userExists__WEBPACK_IMPORTED_MODULE_2__[\"default\"], false);\n};\n\n\n//# sourceURL=webpack:///./templates/auth/register/index.js?");

/***/ }),

/***/ "./templates/auth/register/userExists.js":
/*!***********************************************!*\
  !*** ./templates/auth/register/userExists.js ***!
  \***********************************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _api_v1_routes__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../../api/v1/routes */ \"./templates/api/v1/routes.js\");\n/* harmony import */ var _utils_genJsonPayload__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../../utils/genJsonPayload */ \"./templates/utils/genJsonPayload.js\");\n\n\n\n\n\n//export const checkUsernameExists = async () => {\nasync function userExists()  {\n  let username = document.getElementById('username');\n  let val = username.value;\n  let payload = {\n    val,\n  };\n\n  //  return fetch(ROUTES.usernameExists, genJsonPayload(payload)).then(res => {\n  //    if (res.ok) {\n  //      res.json().then(data => {\n  //        if (data.exists) {\n  //          username.className += ' form__in-field--warn';\n  //          alert('Username taken');\n  //        }\n  //        return data.exists;\n  //      });\n  //    } else {\n  //      res.json().then(err => alert(`error: ${err.error}`));\n  //    }\n  //  });\n  //\n\n  let res = await fetch(_api_v1_routes__WEBPACK_IMPORTED_MODULE_0__[\"default\"].usernameExists, Object(_utils_genJsonPayload__WEBPACK_IMPORTED_MODULE_1__[\"default\"])(payload));\n  if (res.ok) {\n    let data = await res.json();\n    if (data.exists) {\n      username.className += ' form__in-field--warn';\n      alert('Username taken');\n    }\n    return data.exists;\n  } else {\n    let err = await res.json();\n    alert(`error: ${err.error}`);\n  }\n  return false;\n};\n\n/* harmony default export */ __webpack_exports__[\"default\"] = (userExists);\n\n\n//# sourceURL=webpack:///./templates/auth/register/userExists.js?");

/***/ }),

/***/ "./templates/index.js":
/*!****************************!*\
  !*** ./templates/index.js ***!
  \****************************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _router__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./router */ \"./templates/router.js\");\n/* harmony import */ var _auth_login__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./auth/login */ \"./templates/auth/login/index.js\");\n/* harmony import */ var _auth_register__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./auth/register */ \"./templates/auth/register/index.js\");\n/* harmony import */ var _panel_index__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./panel/index */ \"./templates/panel/index.js\");\n\n\n\n\n\n\nconst router = new _router__WEBPACK_IMPORTED_MODULE_0__[\"Router\"]();\n\nrouter.register('/', _auth_login__WEBPACK_IMPORTED_MODULE_1__[\"index\"]);\nrouter.register('/join', _auth_register__WEBPACK_IMPORTED_MODULE_2__[\"index\"]);\nrouter.register('/panel/', _panel_index__WEBPACK_IMPORTED_MODULE_3__[\"index\"]);\nrouter.register('/panel/layout.html/', _panel_index__WEBPACK_IMPORTED_MODULE_3__[\"index\"]);\n\nrouter.route();\n\n\n//# sourceURL=webpack:///./templates/index.js?");

/***/ }),

/***/ "./templates/panel/index.js":
/*!**********************************!*\
  !*** ./templates/panel/index.js ***!
  \**********************************/
/*! exports provided: index */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"index\", function() { return index; });\n/*\nimport './main.scss';\nimport './nav/main.scss';\n*/\n\nconst index = () => {\n  //  const html = document.documentElement;\n  //  const body = document.body;\n  //  const menuLinks = document.querySelectorAll('.admin-menu a');\n  //  const collapseBtn = document.querySelector('.admin-menu .collapse-btn');\n  //  const toggleMobileMenu = document.querySelector('.toggle-mob-menu');\n  //  const switchInput = document.querySelector('.switch input');\n  //  const switchLabel = document.querySelector('.switch label');\n  //  const switchLabelText = switchLabel.querySelector('span:last-child');\n  //  const collapsedClass = 'collapsed';\n  //  const lightModeClass = 'light-mode';\n  //\n  //  /*TOGGLE HEADER STATE*/\n  //  collapseBtn.addEventListener('click', function() {\n  //    body.classList.toggle(collapsedClass);\n  //    this.getAttribute('aria-expanded') == 'true'\n  //      ? this.setAttribute('aria-expanded', 'false')\n  //      : this.setAttribute('aria-expanded', 'true');\n  //    this.getAttribute('aria-label') == 'collapse menu'\n  //      ? this.setAttribute('aria-label', 'expand menu')\n  //      : this.setAttribute('aria-label', 'collapse menu');\n  //  });\n  //\n  //  /*TOGGLE MOBILE MENU*/\n  //  toggleMobileMenu.addEventListener('click', function() {\n  //    body.classList.toggle('mob-menu-opened');\n  //    this.getAttribute('aria-expanded') == 'true'\n  //      ? this.setAttribute('aria-expanded', 'false')\n  //      : this.setAttribute('aria-expanded', 'true');\n  //    this.getAttribute('aria-label') == 'open menu'\n  //      ? this.setAttribute('aria-label', 'close menu')\n  //      : this.setAttribute('aria-label', 'open menu');\n  //  });\n  //\n  //  /*SHOW TOOLTIP ON MENU LINK HOVER*/\n  //  for (const link of menuLinks) {\n  //    link.addEventListener('mouseenter', function() {\n  //      if (\n  //        body.classList.contains(collapsedClass) &&\n  //        window.matchMedia('(min-width: 768px)').matches\n  //      ) {\n  //        const tooltip = this.querySelector('span').textContent;\n  //        this.setAttribute('title', tooltip);\n  //      } else {\n  //        this.removeAttribute('title');\n  //      }\n  //    });\n  //  }\n  //\n  //  /*TOGGLE LIGHT/DARK MODE*/\n  //  if (localStorage.getItem('dark-mode') === 'false') {\n  //    html.classList.add(lightModeClass);\n  //    switchInput.checked = false;\n  //    switchLabelText.textContent = 'Light';\n  //  }\n  //\n  //  switchInput.addEventListener('input', function() {\n  //    html.classList.toggle(lightModeClass);\n  //    if (html.classList.contains(lightModeClass)) {\n  //      switchLabelText.textContent = 'Light';\n  //      localStorage.setItem('dark-mode', 'false');\n  //    } else {\n  //      switchLabelText.textContent = 'Dark';\n  //      localStorage.setItem('dark-mode', 'true');\n  //    }\n  //  });\n  //\n  let a;\n};\n\n\n//# sourceURL=webpack:///./templates/panel/index.js?");

/***/ }),

/***/ "./templates/router.js":
/*!*****************************!*\
  !*** ./templates/router.js ***!
  \*****************************/
/*! exports provided: Router */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Router\", function() { return Router; });\nconst normalizeUri = uri => {\n  if (!uri) {\n    throw new Error('uri is empty');\n  }\n\n  if (typeof uri !== 'string') {\n    throw new TypeError('URI must be a string');\n  }\n\n  let uriLength = uri.length;\n  if (uri[uriLength - 1] == '/') {\n    uri = uri.slice(0, uriLength - 1);\n  }\n  return uri;\n};\n\nclass Router {\n  constructor() {\n    this.routes = [];\n  }\n\n  register(uri, fn) {\n    // typechecks\n    if (!uri) {\n      throw new Error('uri is empty');\n    }\n\n    if (!fn) {\n      throw new Error('fn is empty');\n    }\n    if (typeof uri !== 'string') {\n      throw new TypeError('URI must be a string');\n    }\n\n    if (typeof fn !== 'function') {\n      throw new TypeError('a callback fn must be provided');\n    }\n\n    this.routes.forEach(route => {\n      if (route.uri == uri) {\n        throw new Error(\n          `URI exists. provided URI: ${uri}, registered config: ${route}`,\n        );\n      }\n    });\n\n    uri = normalizeUri(uri);\n\n    const route = {\n      uri,\n      fn,\n    };\n    this.routes.push(route);\n  }\n\n  route() {\n    this.routes.forEach(route => {\n      // normalize for trailing slash\n      let pattern = new RegExp(`^${route.uri}$`);\n      let path = window.location.pathname;\n      path = normalizeUri(path);\n      if (path.match(pattern)) {\n        return route.fn.call();\n      }\n    });\n  }\n}\n\n\n//# sourceURL=webpack:///./templates/router.js?");

/***/ }),

/***/ "./templates/utils/genJsonPayload.js":
/*!*******************************************!*\
  !*** ./templates/utils/genJsonPayload.js ***!
  \*******************************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\nconst genJsonPayload = payload => {\n  let value = {\n    method: 'POST',\n    headers: {\n      'Content-Type': 'application/json',\n    },\n    body: JSON.stringify(payload),\n  };\n  return value;\n};\n\n/* harmony default export */ __webpack_exports__[\"default\"] = (genJsonPayload);\n\n\n//# sourceURL=webpack:///./templates/utils/genJsonPayload.js?");

/***/ })

/******/ });