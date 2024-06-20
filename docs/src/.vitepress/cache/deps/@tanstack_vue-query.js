import {
  setupDevtoolsPlugin
} from "./chunk-Y2XQL4KM.js";
import {
  isVue2
} from "./chunk-I3RIQLCS.js";
import {
  computed2 as computed,
  getCurrentScope,
  hasInjectionContext,
  inject,
  isRef,
  onScopeDispose,
  reactive,
  readonly,
  ref,
  toRefs,
  unref,
  watch,
  watchEffect
} from "./chunk-T3FA6UVC.js";
import {
  __privateAdd,
  __privateGet,
  __privateMethod,
  __privateSet,
  __privateWrapper
} from "./chunk-HL2QZUHZ.js";

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/subscribable.js
var Subscribable = class {
  constructor() {
    this.listeners = /* @__PURE__ */ new Set();
    this.subscribe = this.subscribe.bind(this);
  }
  subscribe(listener) {
    this.listeners.add(listener);
    this.onSubscribe();
    return () => {
      this.listeners.delete(listener);
      this.onUnsubscribe();
    };
  }
  hasListeners() {
    return this.listeners.size > 0;
  }
  onSubscribe() {
  }
  onUnsubscribe() {
  }
};

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/utils.js
var isServer = typeof window === "undefined" || "Deno" in window;
function noop() {
  return void 0;
}
function functionalUpdate(updater, input) {
  return typeof updater === "function" ? updater(input) : updater;
}
function isValidTimeout(value) {
  return typeof value === "number" && value >= 0 && value !== Infinity;
}
function timeUntilStale(updatedAt, staleTime) {
  return Math.max(updatedAt + (staleTime || 0) - Date.now(), 0);
}
function matchQuery(filters, query) {
  const {
    type = "all",
    exact,
    fetchStatus,
    predicate,
    queryKey,
    stale
  } = filters;
  if (queryKey) {
    if (exact) {
      if (query.queryHash !== hashQueryKeyByOptions(queryKey, query.options)) {
        return false;
      }
    } else if (!partialMatchKey(query.queryKey, queryKey)) {
      return false;
    }
  }
  if (type !== "all") {
    const isActive = query.isActive();
    if (type === "active" && !isActive) {
      return false;
    }
    if (type === "inactive" && isActive) {
      return false;
    }
  }
  if (typeof stale === "boolean" && query.isStale() !== stale) {
    return false;
  }
  if (typeof fetchStatus !== "undefined" && fetchStatus !== query.state.fetchStatus) {
    return false;
  }
  if (predicate && !predicate(query)) {
    return false;
  }
  return true;
}
function matchMutation(filters, mutation) {
  const { exact, status, predicate, mutationKey } = filters;
  if (mutationKey) {
    if (!mutation.options.mutationKey) {
      return false;
    }
    if (exact) {
      if (hashKey(mutation.options.mutationKey) !== hashKey(mutationKey)) {
        return false;
      }
    } else if (!partialMatchKey(mutation.options.mutationKey, mutationKey)) {
      return false;
    }
  }
  if (status && mutation.state.status !== status) {
    return false;
  }
  if (predicate && !predicate(mutation)) {
    return false;
  }
  return true;
}
function hashQueryKeyByOptions(queryKey, options) {
  const hashFn = (options == null ? void 0 : options.queryKeyHashFn) || hashKey;
  return hashFn(queryKey);
}
function hashKey(queryKey) {
  return JSON.stringify(
    queryKey,
    (_, val) => isPlainObject(val) ? Object.keys(val).sort().reduce((result, key) => {
      result[key] = val[key];
      return result;
    }, {}) : val
  );
}
function partialMatchKey(a, b) {
  if (a === b) {
    return true;
  }
  if (typeof a !== typeof b) {
    return false;
  }
  if (a && b && typeof a === "object" && typeof b === "object") {
    return !Object.keys(b).some((key) => !partialMatchKey(a[key], b[key]));
  }
  return false;
}
function replaceEqualDeep(a, b) {
  if (a === b) {
    return a;
  }
  const array = isPlainArray(a) && isPlainArray(b);
  if (array || isPlainObject(a) && isPlainObject(b)) {
    const aSize = array ? a.length : Object.keys(a).length;
    const bItems = array ? b : Object.keys(b);
    const bSize = bItems.length;
    const copy = array ? [] : {};
    let equalItems = 0;
    for (let i = 0; i < bSize; i++) {
      const key = array ? i : bItems[i];
      copy[key] = replaceEqualDeep(a[key], b[key]);
      if (copy[key] === a[key]) {
        equalItems++;
      }
    }
    return aSize === bSize && equalItems === aSize ? a : copy;
  }
  return b;
}
function shallowEqualObjects(a, b) {
  if (a && !b || b && !a) {
    return false;
  }
  for (const key in a) {
    if (a[key] !== b[key]) {
      return false;
    }
  }
  return true;
}
function isPlainArray(value) {
  return Array.isArray(value) && value.length === Object.keys(value).length;
}
function isPlainObject(o) {
  if (!hasObjectPrototype(o)) {
    return false;
  }
  const ctor = o.constructor;
  if (typeof ctor === "undefined") {
    return true;
  }
  const prot = ctor.prototype;
  if (!hasObjectPrototype(prot)) {
    return false;
  }
  if (!prot.hasOwnProperty("isPrototypeOf")) {
    return false;
  }
  return true;
}
function hasObjectPrototype(o) {
  return Object.prototype.toString.call(o) === "[object Object]";
}
function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}
function replaceData(prevData, data, options) {
  if (typeof options.structuralSharing === "function") {
    return options.structuralSharing(prevData, data);
  } else if (options.structuralSharing !== false) {
    return replaceEqualDeep(prevData, data);
  }
  return data;
}
function keepPreviousData(previousData) {
  return previousData;
}
function addToEnd(items, item, max = 0) {
  const newItems = [...items, item];
  return max && newItems.length > max ? newItems.slice(1) : newItems;
}
function addToStart(items, item, max = 0) {
  const newItems = [item, ...items];
  return max && newItems.length > max ? newItems.slice(0, -1) : newItems;
}

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/focusManager.js
var _focused, _cleanup, _setup, _a;
var FocusManager = (_a = class extends Subscribable {
  constructor() {
    super();
    __privateAdd(this, _focused, void 0);
    __privateAdd(this, _cleanup, void 0);
    __privateAdd(this, _setup, void 0);
    __privateSet(this, _setup, (onFocus) => {
      if (!isServer && window.addEventListener) {
        const listener = () => onFocus();
        window.addEventListener("visibilitychange", listener, false);
        return () => {
          window.removeEventListener("visibilitychange", listener);
        };
      }
      return;
    });
  }
  onSubscribe() {
    if (!__privateGet(this, _cleanup)) {
      this.setEventListener(__privateGet(this, _setup));
    }
  }
  onUnsubscribe() {
    var _a12;
    if (!this.hasListeners()) {
      (_a12 = __privateGet(this, _cleanup)) == null ? void 0 : _a12.call(this);
      __privateSet(this, _cleanup, void 0);
    }
  }
  setEventListener(setup) {
    var _a12;
    __privateSet(this, _setup, setup);
    (_a12 = __privateGet(this, _cleanup)) == null ? void 0 : _a12.call(this);
    __privateSet(this, _cleanup, setup((focused) => {
      if (typeof focused === "boolean") {
        this.setFocused(focused);
      } else {
        this.onFocus();
      }
    }));
  }
  setFocused(focused) {
    const changed = __privateGet(this, _focused) !== focused;
    if (changed) {
      __privateSet(this, _focused, focused);
      this.onFocus();
    }
  }
  onFocus() {
    this.listeners.forEach((listener) => {
      listener();
    });
  }
  isFocused() {
    var _a12;
    if (typeof __privateGet(this, _focused) === "boolean") {
      return __privateGet(this, _focused);
    }
    return ((_a12 = globalThis.document) == null ? void 0 : _a12.visibilityState) !== "hidden";
  }
}, _focused = new WeakMap(), _cleanup = new WeakMap(), _setup = new WeakMap(), _a);
var focusManager = new FocusManager();

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/onlineManager.js
var _online, _cleanup2, _setup2, _a2;
var OnlineManager = (_a2 = class extends Subscribable {
  constructor() {
    super();
    __privateAdd(this, _online, true);
    __privateAdd(this, _cleanup2, void 0);
    __privateAdd(this, _setup2, void 0);
    __privateSet(this, _setup2, (onOnline) => {
      if (!isServer && window.addEventListener) {
        const onlineListener = () => onOnline(true);
        const offlineListener = () => onOnline(false);
        window.addEventListener("online", onlineListener, false);
        window.addEventListener("offline", offlineListener, false);
        return () => {
          window.removeEventListener("online", onlineListener);
          window.removeEventListener("offline", offlineListener);
        };
      }
      return;
    });
  }
  onSubscribe() {
    if (!__privateGet(this, _cleanup2)) {
      this.setEventListener(__privateGet(this, _setup2));
    }
  }
  onUnsubscribe() {
    var _a12;
    if (!this.hasListeners()) {
      (_a12 = __privateGet(this, _cleanup2)) == null ? void 0 : _a12.call(this);
      __privateSet(this, _cleanup2, void 0);
    }
  }
  setEventListener(setup) {
    var _a12;
    __privateSet(this, _setup2, setup);
    (_a12 = __privateGet(this, _cleanup2)) == null ? void 0 : _a12.call(this);
    __privateSet(this, _cleanup2, setup(this.setOnline.bind(this)));
  }
  setOnline(online) {
    const changed = __privateGet(this, _online) !== online;
    if (changed) {
      __privateSet(this, _online, online);
      this.listeners.forEach((listener) => {
        listener(online);
      });
    }
  }
  isOnline() {
    return __privateGet(this, _online);
  }
}, _online = new WeakMap(), _cleanup2 = new WeakMap(), _setup2 = new WeakMap(), _a2);
var onlineManager = new OnlineManager();

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/retryer.js
function defaultRetryDelay(failureCount) {
  return Math.min(1e3 * 2 ** failureCount, 3e4);
}
function canFetch(networkMode) {
  return (networkMode ?? "online") === "online" ? onlineManager.isOnline() : true;
}
var CancelledError = class {
  constructor(options) {
    this.revert = options == null ? void 0 : options.revert;
    this.silent = options == null ? void 0 : options.silent;
  }
};
function isCancelledError(value) {
  return value instanceof CancelledError;
}
function createRetryer(config) {
  let isRetryCancelled = false;
  let failureCount = 0;
  let isResolved = false;
  let continueFn;
  let promiseResolve;
  let promiseReject;
  const promise = new Promise((outerResolve, outerReject) => {
    promiseResolve = outerResolve;
    promiseReject = outerReject;
  });
  const cancel = (cancelOptions) => {
    var _a12;
    if (!isResolved) {
      reject(new CancelledError(cancelOptions));
      (_a12 = config.abort) == null ? void 0 : _a12.call(config);
    }
  };
  const cancelRetry = () => {
    isRetryCancelled = true;
  };
  const continueRetry = () => {
    isRetryCancelled = false;
  };
  const shouldPause = () => !focusManager.isFocused() || config.networkMode !== "always" && !onlineManager.isOnline();
  const resolve = (value) => {
    var _a12;
    if (!isResolved) {
      isResolved = true;
      (_a12 = config.onSuccess) == null ? void 0 : _a12.call(config, value);
      continueFn == null ? void 0 : continueFn();
      promiseResolve(value);
    }
  };
  const reject = (value) => {
    var _a12;
    if (!isResolved) {
      isResolved = true;
      (_a12 = config.onError) == null ? void 0 : _a12.call(config, value);
      continueFn == null ? void 0 : continueFn();
      promiseReject(value);
    }
  };
  const pause = () => {
    return new Promise((continueResolve) => {
      var _a12;
      continueFn = (value) => {
        const canContinue = isResolved || !shouldPause();
        if (canContinue) {
          continueResolve(value);
        }
        return canContinue;
      };
      (_a12 = config.onPause) == null ? void 0 : _a12.call(config);
    }).then(() => {
      var _a12;
      continueFn = void 0;
      if (!isResolved) {
        (_a12 = config.onContinue) == null ? void 0 : _a12.call(config);
      }
    });
  };
  const run = () => {
    if (isResolved) {
      return;
    }
    let promiseOrValue;
    try {
      promiseOrValue = config.fn();
    } catch (error) {
      promiseOrValue = Promise.reject(error);
    }
    Promise.resolve(promiseOrValue).then(resolve).catch((error) => {
      var _a12;
      if (isResolved) {
        return;
      }
      const retry = config.retry ?? (isServer ? 0 : 3);
      const retryDelay = config.retryDelay ?? defaultRetryDelay;
      const delay = typeof retryDelay === "function" ? retryDelay(failureCount, error) : retryDelay;
      const shouldRetry = retry === true || typeof retry === "number" && failureCount < retry || typeof retry === "function" && retry(failureCount, error);
      if (isRetryCancelled || !shouldRetry) {
        reject(error);
        return;
      }
      failureCount++;
      (_a12 = config.onFail) == null ? void 0 : _a12.call(config, failureCount, error);
      sleep(delay).then(() => {
        if (shouldPause()) {
          return pause();
        }
        return;
      }).then(() => {
        if (isRetryCancelled) {
          reject(error);
        } else {
          run();
        }
      });
    });
  };
  if (canFetch(config.networkMode)) {
    run();
  } else {
    pause().then(run);
  }
  return {
    promise,
    cancel,
    continue: () => {
      const didContinue = continueFn == null ? void 0 : continueFn();
      return didContinue ? promise : Promise.resolve();
    },
    cancelRetry,
    continueRetry
  };
}

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/notifyManager.js
function createNotifyManager() {
  let queue = [];
  let transactions = 0;
  let notifyFn = (callback) => {
    callback();
  };
  let batchNotifyFn = (callback) => {
    callback();
  };
  let scheduleFn = (cb) => setTimeout(cb, 0);
  const setScheduler = (fn) => {
    scheduleFn = fn;
  };
  const batch = (callback) => {
    let result;
    transactions++;
    try {
      result = callback();
    } finally {
      transactions--;
      if (!transactions) {
        flush();
      }
    }
    return result;
  };
  const schedule = (callback) => {
    if (transactions) {
      queue.push(callback);
    } else {
      scheduleFn(() => {
        notifyFn(callback);
      });
    }
  };
  const batchCalls = (callback) => {
    return (...args) => {
      schedule(() => {
        callback(...args);
      });
    };
  };
  const flush = () => {
    const originalQueue = queue;
    queue = [];
    if (originalQueue.length) {
      scheduleFn(() => {
        batchNotifyFn(() => {
          originalQueue.forEach((callback) => {
            notifyFn(callback);
          });
        });
      });
    }
  };
  const setNotifyFunction = (fn) => {
    notifyFn = fn;
  };
  const setBatchNotifyFunction = (fn) => {
    batchNotifyFn = fn;
  };
  return {
    batch,
    batchCalls,
    schedule,
    setNotifyFunction,
    setBatchNotifyFunction,
    setScheduler
  };
}
var notifyManager = createNotifyManager();

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/removable.js
var _gcTimeout, _a3;
var Removable = (_a3 = class {
  constructor() {
    __privateAdd(this, _gcTimeout, void 0);
  }
  destroy() {
    this.clearGcTimeout();
  }
  scheduleGc() {
    this.clearGcTimeout();
    if (isValidTimeout(this.gcTime)) {
      __privateSet(this, _gcTimeout, setTimeout(() => {
        this.optionalRemove();
      }, this.gcTime));
    }
  }
  updateGcTime(newGcTime) {
    this.gcTime = Math.max(
      this.gcTime || 0,
      newGcTime ?? (isServer ? Infinity : 5 * 60 * 1e3)
    );
  }
  clearGcTimeout() {
    if (__privateGet(this, _gcTimeout)) {
      clearTimeout(__privateGet(this, _gcTimeout));
      __privateSet(this, _gcTimeout, void 0);
    }
  }
}, _gcTimeout = new WeakMap(), _a3);

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/query.js
var _initialState, _revertState, _cache, _promise, _retryer, _observers, _defaultOptions, _abortSignalConsumed, _setOptions, setOptions_fn, _dispatch, dispatch_fn, _a4;
var Query = (_a4 = class extends Removable {
  constructor(config) {
    super();
    __privateAdd(this, _setOptions);
    __privateAdd(this, _dispatch);
    __privateAdd(this, _initialState, void 0);
    __privateAdd(this, _revertState, void 0);
    __privateAdd(this, _cache, void 0);
    __privateAdd(this, _promise, void 0);
    __privateAdd(this, _retryer, void 0);
    __privateAdd(this, _observers, void 0);
    __privateAdd(this, _defaultOptions, void 0);
    __privateAdd(this, _abortSignalConsumed, void 0);
    __privateSet(this, _abortSignalConsumed, false);
    __privateSet(this, _defaultOptions, config.defaultOptions);
    __privateMethod(this, _setOptions, setOptions_fn).call(this, config.options);
    __privateSet(this, _observers, []);
    __privateSet(this, _cache, config.cache);
    this.queryKey = config.queryKey;
    this.queryHash = config.queryHash;
    __privateSet(this, _initialState, config.state || getDefaultState(this.options));
    this.state = __privateGet(this, _initialState);
    this.scheduleGc();
  }
  get meta() {
    return this.options.meta;
  }
  optionalRemove() {
    if (!__privateGet(this, _observers).length && this.state.fetchStatus === "idle") {
      __privateGet(this, _cache).remove(this);
    }
  }
  setData(newData, options) {
    const data = replaceData(this.state.data, newData, this.options);
    __privateMethod(this, _dispatch, dispatch_fn).call(this, {
      data,
      type: "success",
      dataUpdatedAt: options == null ? void 0 : options.updatedAt,
      manual: options == null ? void 0 : options.manual
    });
    return data;
  }
  setState(state, setStateOptions) {
    __privateMethod(this, _dispatch, dispatch_fn).call(this, { type: "setState", state, setStateOptions });
  }
  cancel(options) {
    var _a12;
    const promise = __privateGet(this, _promise);
    (_a12 = __privateGet(this, _retryer)) == null ? void 0 : _a12.cancel(options);
    return promise ? promise.then(noop).catch(noop) : Promise.resolve();
  }
  destroy() {
    super.destroy();
    this.cancel({ silent: true });
  }
  reset() {
    this.destroy();
    this.setState(__privateGet(this, _initialState));
  }
  isActive() {
    return __privateGet(this, _observers).some(
      (observer) => observer.options.enabled !== false
    );
  }
  isDisabled() {
    return this.getObserversCount() > 0 && !this.isActive();
  }
  isStale() {
    return this.state.isInvalidated || !this.state.dataUpdatedAt || __privateGet(this, _observers).some((observer) => observer.getCurrentResult().isStale);
  }
  isStaleByTime(staleTime = 0) {
    return this.state.isInvalidated || !this.state.dataUpdatedAt || !timeUntilStale(this.state.dataUpdatedAt, staleTime);
  }
  onFocus() {
    var _a12;
    const observer = __privateGet(this, _observers).find((x) => x.shouldFetchOnWindowFocus());
    observer == null ? void 0 : observer.refetch({ cancelRefetch: false });
    (_a12 = __privateGet(this, _retryer)) == null ? void 0 : _a12.continue();
  }
  onOnline() {
    var _a12;
    const observer = __privateGet(this, _observers).find((x) => x.shouldFetchOnReconnect());
    observer == null ? void 0 : observer.refetch({ cancelRefetch: false });
    (_a12 = __privateGet(this, _retryer)) == null ? void 0 : _a12.continue();
  }
  addObserver(observer) {
    if (!__privateGet(this, _observers).includes(observer)) {
      __privateGet(this, _observers).push(observer);
      this.clearGcTimeout();
      __privateGet(this, _cache).notify({ type: "observerAdded", query: this, observer });
    }
  }
  removeObserver(observer) {
    if (__privateGet(this, _observers).includes(observer)) {
      __privateSet(this, _observers, __privateGet(this, _observers).filter((x) => x !== observer));
      if (!__privateGet(this, _observers).length) {
        if (__privateGet(this, _retryer)) {
          if (__privateGet(this, _abortSignalConsumed)) {
            __privateGet(this, _retryer).cancel({ revert: true });
          } else {
            __privateGet(this, _retryer).cancelRetry();
          }
        }
        this.scheduleGc();
      }
      __privateGet(this, _cache).notify({ type: "observerRemoved", query: this, observer });
    }
  }
  getObserversCount() {
    return __privateGet(this, _observers).length;
  }
  invalidate() {
    if (!this.state.isInvalidated) {
      __privateMethod(this, _dispatch, dispatch_fn).call(this, { type: "invalidate" });
    }
  }
  fetch(options, fetchOptions) {
    var _a12, _b, _c, _d;
    if (this.state.fetchStatus !== "idle") {
      if (this.state.dataUpdatedAt && (fetchOptions == null ? void 0 : fetchOptions.cancelRefetch)) {
        this.cancel({ silent: true });
      } else if (__privateGet(this, _promise)) {
        (_a12 = __privateGet(this, _retryer)) == null ? void 0 : _a12.continueRetry();
        return __privateGet(this, _promise);
      }
    }
    if (options) {
      __privateMethod(this, _setOptions, setOptions_fn).call(this, options);
    }
    if (!this.options.queryFn) {
      const observer = __privateGet(this, _observers).find((x) => x.options.queryFn);
      if (observer) {
        __privateMethod(this, _setOptions, setOptions_fn).call(this, observer.options);
      }
    }
    if (true) {
      if (!Array.isArray(this.options.queryKey)) {
        console.error(
          `As of v4, queryKey needs to be an Array. If you are using a string like 'repoData', please change it to an Array, e.g. ['repoData']`
        );
      }
    }
    const abortController = new AbortController();
    const queryFnContext = {
      queryKey: this.queryKey,
      meta: this.meta
    };
    const addSignalProperty = (object) => {
      Object.defineProperty(object, "signal", {
        enumerable: true,
        get: () => {
          __privateSet(this, _abortSignalConsumed, true);
          return abortController.signal;
        }
      });
    };
    addSignalProperty(queryFnContext);
    const fetchFn = () => {
      if (!this.options.queryFn) {
        return Promise.reject(
          new Error(`Missing queryFn: '${this.options.queryHash}'`)
        );
      }
      __privateSet(this, _abortSignalConsumed, false);
      if (this.options.persister) {
        return this.options.persister(
          this.options.queryFn,
          queryFnContext,
          this
        );
      }
      return this.options.queryFn(
        queryFnContext
      );
    };
    const context = {
      fetchOptions,
      options: this.options,
      queryKey: this.queryKey,
      state: this.state,
      fetchFn
    };
    addSignalProperty(context);
    (_b = this.options.behavior) == null ? void 0 : _b.onFetch(
      context,
      this
    );
    __privateSet(this, _revertState, this.state);
    if (this.state.fetchStatus === "idle" || this.state.fetchMeta !== ((_c = context.fetchOptions) == null ? void 0 : _c.meta)) {
      __privateMethod(this, _dispatch, dispatch_fn).call(this, { type: "fetch", meta: (_d = context.fetchOptions) == null ? void 0 : _d.meta });
    }
    const onError = (error) => {
      var _a13, _b2, _c2, _d2;
      if (!(isCancelledError(error) && error.silent)) {
        __privateMethod(this, _dispatch, dispatch_fn).call(this, {
          type: "error",
          error
        });
      }
      if (!isCancelledError(error)) {
        (_b2 = (_a13 = __privateGet(this, _cache).config).onError) == null ? void 0 : _b2.call(
          _a13,
          error,
          this
        );
        (_d2 = (_c2 = __privateGet(this, _cache).config).onSettled) == null ? void 0 : _d2.call(
          _c2,
          this.state.data,
          error,
          this
        );
      }
      if (!this.isFetchingOptimistic) {
        this.scheduleGc();
      }
      this.isFetchingOptimistic = false;
    };
    __privateSet(this, _retryer, createRetryer({
      fn: context.fetchFn,
      abort: abortController.abort.bind(abortController),
      onSuccess: (data) => {
        var _a13, _b2, _c2, _d2;
        if (typeof data === "undefined") {
          if (true) {
            console.error(
              `Query data cannot be undefined. Please make sure to return a value other than undefined from your query function. Affected query key: ${this.queryHash}`
            );
          }
          onError(new Error(`${this.queryHash} data is undefined`));
          return;
        }
        this.setData(data);
        (_b2 = (_a13 = __privateGet(this, _cache).config).onSuccess) == null ? void 0 : _b2.call(_a13, data, this);
        (_d2 = (_c2 = __privateGet(this, _cache).config).onSettled) == null ? void 0 : _d2.call(
          _c2,
          data,
          this.state.error,
          this
        );
        if (!this.isFetchingOptimistic) {
          this.scheduleGc();
        }
        this.isFetchingOptimistic = false;
      },
      onError,
      onFail: (failureCount, error) => {
        __privateMethod(this, _dispatch, dispatch_fn).call(this, { type: "failed", failureCount, error });
      },
      onPause: () => {
        __privateMethod(this, _dispatch, dispatch_fn).call(this, { type: "pause" });
      },
      onContinue: () => {
        __privateMethod(this, _dispatch, dispatch_fn).call(this, { type: "continue" });
      },
      retry: context.options.retry,
      retryDelay: context.options.retryDelay,
      networkMode: context.options.networkMode
    }));
    __privateSet(this, _promise, __privateGet(this, _retryer).promise);
    return __privateGet(this, _promise);
  }
}, _initialState = new WeakMap(), _revertState = new WeakMap(), _cache = new WeakMap(), _promise = new WeakMap(), _retryer = new WeakMap(), _observers = new WeakMap(), _defaultOptions = new WeakMap(), _abortSignalConsumed = new WeakMap(), _setOptions = new WeakSet(), setOptions_fn = function(options) {
  this.options = { ...__privateGet(this, _defaultOptions), ...options };
  this.updateGcTime(this.options.gcTime);
}, _dispatch = new WeakSet(), dispatch_fn = function(action) {
  const reducer = (state) => {
    switch (action.type) {
      case "failed":
        return {
          ...state,
          fetchFailureCount: action.failureCount,
          fetchFailureReason: action.error
        };
      case "pause":
        return {
          ...state,
          fetchStatus: "paused"
        };
      case "continue":
        return {
          ...state,
          fetchStatus: "fetching"
        };
      case "fetch":
        return {
          ...state,
          fetchFailureCount: 0,
          fetchFailureReason: null,
          fetchMeta: action.meta ?? null,
          fetchStatus: canFetch(this.options.networkMode) ? "fetching" : "paused",
          ...!state.dataUpdatedAt && {
            error: null,
            status: "pending"
          }
        };
      case "success":
        return {
          ...state,
          data: action.data,
          dataUpdateCount: state.dataUpdateCount + 1,
          dataUpdatedAt: action.dataUpdatedAt ?? Date.now(),
          error: null,
          isInvalidated: false,
          status: "success",
          ...!action.manual && {
            fetchStatus: "idle",
            fetchFailureCount: 0,
            fetchFailureReason: null
          }
        };
      case "error":
        const error = action.error;
        if (isCancelledError(error) && error.revert && __privateGet(this, _revertState)) {
          return { ...__privateGet(this, _revertState), fetchStatus: "idle" };
        }
        return {
          ...state,
          error,
          errorUpdateCount: state.errorUpdateCount + 1,
          errorUpdatedAt: Date.now(),
          fetchFailureCount: state.fetchFailureCount + 1,
          fetchFailureReason: error,
          fetchStatus: "idle",
          status: "error"
        };
      case "invalidate":
        return {
          ...state,
          isInvalidated: true
        };
      case "setState":
        return {
          ...state,
          ...action.state
        };
    }
  };
  this.state = reducer(this.state);
  notifyManager.batch(() => {
    __privateGet(this, _observers).forEach((observer) => {
      observer.onQueryUpdate();
    });
    __privateGet(this, _cache).notify({ query: this, type: "updated", action });
  });
}, _a4);
function getDefaultState(options) {
  const data = typeof options.initialData === "function" ? options.initialData() : options.initialData;
  const hasData = typeof data !== "undefined";
  const initialDataUpdatedAt = hasData ? typeof options.initialDataUpdatedAt === "function" ? options.initialDataUpdatedAt() : options.initialDataUpdatedAt : 0;
  return {
    data,
    dataUpdateCount: 0,
    dataUpdatedAt: hasData ? initialDataUpdatedAt ?? Date.now() : 0,
    error: null,
    errorUpdateCount: 0,
    errorUpdatedAt: 0,
    fetchFailureCount: 0,
    fetchFailureReason: null,
    fetchMeta: null,
    isInvalidated: false,
    status: hasData ? "success" : "pending",
    fetchStatus: "idle"
  };
}

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/queryCache.js
var _queries, _a5;
var QueryCache = (_a5 = class extends Subscribable {
  constructor(config = {}) {
    super();
    __privateAdd(this, _queries, void 0);
    this.config = config;
    __privateSet(this, _queries, /* @__PURE__ */ new Map());
  }
  build(client, options, state) {
    const queryKey = options.queryKey;
    const queryHash = options.queryHash ?? hashQueryKeyByOptions(queryKey, options);
    let query = this.get(queryHash);
    if (!query) {
      query = new Query({
        cache: this,
        queryKey,
        queryHash,
        options: client.defaultQueryOptions(options),
        state,
        defaultOptions: client.getQueryDefaults(queryKey)
      });
      this.add(query);
    }
    return query;
  }
  add(query) {
    if (!__privateGet(this, _queries).has(query.queryHash)) {
      __privateGet(this, _queries).set(query.queryHash, query);
      this.notify({
        type: "added",
        query
      });
    }
  }
  remove(query) {
    const queryInMap = __privateGet(this, _queries).get(query.queryHash);
    if (queryInMap) {
      query.destroy();
      if (queryInMap === query) {
        __privateGet(this, _queries).delete(query.queryHash);
      }
      this.notify({ type: "removed", query });
    }
  }
  clear() {
    notifyManager.batch(() => {
      this.getAll().forEach((query) => {
        this.remove(query);
      });
    });
  }
  get(queryHash) {
    return __privateGet(this, _queries).get(queryHash);
  }
  getAll() {
    return [...__privateGet(this, _queries).values()];
  }
  find(filters) {
    const defaultedFilters = { exact: true, ...filters };
    return this.getAll().find(
      (query) => matchQuery(defaultedFilters, query)
    );
  }
  findAll(filters = {}) {
    const queries = this.getAll();
    return Object.keys(filters).length > 0 ? queries.filter((query) => matchQuery(filters, query)) : queries;
  }
  notify(event) {
    notifyManager.batch(() => {
      this.listeners.forEach((listener) => {
        listener(event);
      });
    });
  }
  onFocus() {
    notifyManager.batch(() => {
      this.getAll().forEach((query) => {
        query.onFocus();
      });
    });
  }
  onOnline() {
    notifyManager.batch(() => {
      this.getAll().forEach((query) => {
        query.onOnline();
      });
    });
  }
}, _queries = new WeakMap(), _a5);

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/mutation.js
var _observers2, _defaultOptions2, _mutationCache, _retryer2, _dispatch2, dispatch_fn2, _a6;
var Mutation = (_a6 = class extends Removable {
  constructor(config) {
    super();
    __privateAdd(this, _dispatch2);
    __privateAdd(this, _observers2, void 0);
    __privateAdd(this, _defaultOptions2, void 0);
    __privateAdd(this, _mutationCache, void 0);
    __privateAdd(this, _retryer2, void 0);
    this.mutationId = config.mutationId;
    __privateSet(this, _defaultOptions2, config.defaultOptions);
    __privateSet(this, _mutationCache, config.mutationCache);
    __privateSet(this, _observers2, []);
    this.state = config.state || getDefaultState2();
    this.setOptions(config.options);
    this.scheduleGc();
  }
  setOptions(options) {
    this.options = { ...__privateGet(this, _defaultOptions2), ...options };
    this.updateGcTime(this.options.gcTime);
  }
  get meta() {
    return this.options.meta;
  }
  addObserver(observer) {
    if (!__privateGet(this, _observers2).includes(observer)) {
      __privateGet(this, _observers2).push(observer);
      this.clearGcTimeout();
      __privateGet(this, _mutationCache).notify({
        type: "observerAdded",
        mutation: this,
        observer
      });
    }
  }
  removeObserver(observer) {
    __privateSet(this, _observers2, __privateGet(this, _observers2).filter((x) => x !== observer));
    this.scheduleGc();
    __privateGet(this, _mutationCache).notify({
      type: "observerRemoved",
      mutation: this,
      observer
    });
  }
  optionalRemove() {
    if (!__privateGet(this, _observers2).length) {
      if (this.state.status === "pending") {
        this.scheduleGc();
      } else {
        __privateGet(this, _mutationCache).remove(this);
      }
    }
  }
  continue() {
    var _a12;
    return ((_a12 = __privateGet(this, _retryer2)) == null ? void 0 : _a12.continue()) ?? // continuing a mutation assumes that variables are set, mutation must have been dehydrated before
    this.execute(this.state.variables);
  }
  async execute(variables) {
    var _a12, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o, _p, _q, _r, _s, _t;
    const executeMutation = () => {
      __privateSet(this, _retryer2, createRetryer({
        fn: () => {
          if (!this.options.mutationFn) {
            return Promise.reject(new Error("No mutationFn found"));
          }
          return this.options.mutationFn(variables);
        },
        onFail: (failureCount, error) => {
          __privateMethod(this, _dispatch2, dispatch_fn2).call(this, { type: "failed", failureCount, error });
        },
        onPause: () => {
          __privateMethod(this, _dispatch2, dispatch_fn2).call(this, { type: "pause" });
        },
        onContinue: () => {
          __privateMethod(this, _dispatch2, dispatch_fn2).call(this, { type: "continue" });
        },
        retry: this.options.retry ?? 0,
        retryDelay: this.options.retryDelay,
        networkMode: this.options.networkMode
      }));
      return __privateGet(this, _retryer2).promise;
    };
    const restored = this.state.status === "pending";
    try {
      if (!restored) {
        __privateMethod(this, _dispatch2, dispatch_fn2).call(this, { type: "pending", variables });
        await ((_b = (_a12 = __privateGet(this, _mutationCache).config).onMutate) == null ? void 0 : _b.call(
          _a12,
          variables,
          this
        ));
        const context = await ((_d = (_c = this.options).onMutate) == null ? void 0 : _d.call(_c, variables));
        if (context !== this.state.context) {
          __privateMethod(this, _dispatch2, dispatch_fn2).call(this, {
            type: "pending",
            context,
            variables
          });
        }
      }
      const data = await executeMutation();
      await ((_f = (_e = __privateGet(this, _mutationCache).config).onSuccess) == null ? void 0 : _f.call(
        _e,
        data,
        variables,
        this.state.context,
        this
      ));
      await ((_h = (_g = this.options).onSuccess) == null ? void 0 : _h.call(_g, data, variables, this.state.context));
      await ((_j = (_i = __privateGet(this, _mutationCache).config).onSettled) == null ? void 0 : _j.call(
        _i,
        data,
        null,
        this.state.variables,
        this.state.context,
        this
      ));
      await ((_l = (_k = this.options).onSettled) == null ? void 0 : _l.call(_k, data, null, variables, this.state.context));
      __privateMethod(this, _dispatch2, dispatch_fn2).call(this, { type: "success", data });
      return data;
    } catch (error) {
      try {
        await ((_n = (_m = __privateGet(this, _mutationCache).config).onError) == null ? void 0 : _n.call(
          _m,
          error,
          variables,
          this.state.context,
          this
        ));
        await ((_p = (_o = this.options).onError) == null ? void 0 : _p.call(
          _o,
          error,
          variables,
          this.state.context
        ));
        await ((_r = (_q = __privateGet(this, _mutationCache).config).onSettled) == null ? void 0 : _r.call(
          _q,
          void 0,
          error,
          this.state.variables,
          this.state.context,
          this
        ));
        await ((_t = (_s = this.options).onSettled) == null ? void 0 : _t.call(
          _s,
          void 0,
          error,
          variables,
          this.state.context
        ));
        throw error;
      } finally {
        __privateMethod(this, _dispatch2, dispatch_fn2).call(this, { type: "error", error });
      }
    }
  }
}, _observers2 = new WeakMap(), _defaultOptions2 = new WeakMap(), _mutationCache = new WeakMap(), _retryer2 = new WeakMap(), _dispatch2 = new WeakSet(), dispatch_fn2 = function(action) {
  const reducer = (state) => {
    switch (action.type) {
      case "failed":
        return {
          ...state,
          failureCount: action.failureCount,
          failureReason: action.error
        };
      case "pause":
        return {
          ...state,
          isPaused: true
        };
      case "continue":
        return {
          ...state,
          isPaused: false
        };
      case "pending":
        return {
          ...state,
          context: action.context,
          data: void 0,
          failureCount: 0,
          failureReason: null,
          error: null,
          isPaused: !canFetch(this.options.networkMode),
          status: "pending",
          variables: action.variables,
          submittedAt: Date.now()
        };
      case "success":
        return {
          ...state,
          data: action.data,
          failureCount: 0,
          failureReason: null,
          error: null,
          status: "success",
          isPaused: false
        };
      case "error":
        return {
          ...state,
          data: void 0,
          error: action.error,
          failureCount: state.failureCount + 1,
          failureReason: action.error,
          isPaused: false,
          status: "error"
        };
    }
  };
  this.state = reducer(this.state);
  notifyManager.batch(() => {
    __privateGet(this, _observers2).forEach((observer) => {
      observer.onMutationUpdate(action);
    });
    __privateGet(this, _mutationCache).notify({
      mutation: this,
      type: "updated",
      action
    });
  });
}, _a6);
function getDefaultState2() {
  return {
    context: void 0,
    data: void 0,
    error: null,
    failureCount: 0,
    failureReason: null,
    isPaused: false,
    status: "idle",
    variables: void 0,
    submittedAt: 0
  };
}

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/mutationCache.js
var _mutations, _mutationId, _resuming, _a7;
var MutationCache = (_a7 = class extends Subscribable {
  constructor(config = {}) {
    super();
    __privateAdd(this, _mutations, void 0);
    __privateAdd(this, _mutationId, void 0);
    __privateAdd(this, _resuming, void 0);
    this.config = config;
    __privateSet(this, _mutations, []);
    __privateSet(this, _mutationId, 0);
  }
  build(client, options, state) {
    const mutation = new Mutation({
      mutationCache: this,
      mutationId: ++__privateWrapper(this, _mutationId)._,
      options: client.defaultMutationOptions(options),
      state
    });
    this.add(mutation);
    return mutation;
  }
  add(mutation) {
    __privateGet(this, _mutations).push(mutation);
    this.notify({ type: "added", mutation });
  }
  remove(mutation) {
    __privateSet(this, _mutations, __privateGet(this, _mutations).filter((x) => x !== mutation));
    this.notify({ type: "removed", mutation });
  }
  clear() {
    notifyManager.batch(() => {
      __privateGet(this, _mutations).forEach((mutation) => {
        this.remove(mutation);
      });
    });
  }
  getAll() {
    return __privateGet(this, _mutations);
  }
  find(filters) {
    const defaultedFilters = { exact: true, ...filters };
    return __privateGet(this, _mutations).find(
      (mutation) => matchMutation(defaultedFilters, mutation)
    );
  }
  findAll(filters = {}) {
    return __privateGet(this, _mutations).filter(
      (mutation) => matchMutation(filters, mutation)
    );
  }
  notify(event) {
    notifyManager.batch(() => {
      this.listeners.forEach((listener) => {
        listener(event);
      });
    });
  }
  resumePausedMutations() {
    __privateSet(this, _resuming, (__privateGet(this, _resuming) ?? Promise.resolve()).then(() => {
      const pausedMutations = __privateGet(this, _mutations).filter((x) => x.state.isPaused);
      return notifyManager.batch(
        () => pausedMutations.reduce(
          (promise, mutation) => promise.then(() => mutation.continue().catch(noop)),
          Promise.resolve()
        )
      );
    }).then(() => {
      __privateSet(this, _resuming, void 0);
    }));
    return __privateGet(this, _resuming);
  }
}, _mutations = new WeakMap(), _mutationId = new WeakMap(), _resuming = new WeakMap(), _a7);

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/infiniteQueryBehavior.js
function infiniteQueryBehavior(pages) {
  return {
    onFetch: (context, query) => {
      const fetchFn = async () => {
        var _a12, _b, _c, _d, _e;
        const options = context.options;
        const direction = (_c = (_b = (_a12 = context.fetchOptions) == null ? void 0 : _a12.meta) == null ? void 0 : _b.fetchMore) == null ? void 0 : _c.direction;
        const oldPages = ((_d = context.state.data) == null ? void 0 : _d.pages) || [];
        const oldPageParams = ((_e = context.state.data) == null ? void 0 : _e.pageParams) || [];
        const empty = { pages: [], pageParams: [] };
        let cancelled = false;
        const addSignalProperty = (object) => {
          Object.defineProperty(object, "signal", {
            enumerable: true,
            get: () => {
              if (context.signal.aborted) {
                cancelled = true;
              } else {
                context.signal.addEventListener("abort", () => {
                  cancelled = true;
                });
              }
              return context.signal;
            }
          });
        };
        const queryFn = context.options.queryFn || (() => Promise.reject(
          new Error(`Missing queryFn: '${context.options.queryHash}'`)
        ));
        const fetchPage = async (data, param, previous) => {
          if (cancelled) {
            return Promise.reject();
          }
          if (param == null && data.pages.length) {
            return Promise.resolve(data);
          }
          const queryFnContext = {
            queryKey: context.queryKey,
            pageParam: param,
            direction: previous ? "backward" : "forward",
            meta: context.options.meta
          };
          addSignalProperty(queryFnContext);
          const page = await queryFn(
            queryFnContext
          );
          const { maxPages } = context.options;
          const addTo = previous ? addToStart : addToEnd;
          return {
            pages: addTo(data.pages, page, maxPages),
            pageParams: addTo(data.pageParams, param, maxPages)
          };
        };
        let result;
        if (direction && oldPages.length) {
          const previous = direction === "backward";
          const pageParamFn = previous ? getPreviousPageParam : getNextPageParam;
          const oldData = {
            pages: oldPages,
            pageParams: oldPageParams
          };
          const param = pageParamFn(options, oldData);
          result = await fetchPage(oldData, param, previous);
        } else {
          result = await fetchPage(
            empty,
            oldPageParams[0] ?? options.initialPageParam
          );
          const remainingPages = pages ?? oldPages.length;
          for (let i = 1; i < remainingPages; i++) {
            const param = getNextPageParam(options, result);
            result = await fetchPage(result, param);
          }
        }
        return result;
      };
      if (context.options.persister) {
        context.fetchFn = () => {
          var _a12, _b;
          return (_b = (_a12 = context.options).persister) == null ? void 0 : _b.call(
            _a12,
            fetchFn,
            {
              queryKey: context.queryKey,
              meta: context.options.meta,
              signal: context.signal
            },
            query
          );
        };
      } else {
        context.fetchFn = fetchFn;
      }
    }
  };
}
function getNextPageParam(options, { pages, pageParams }) {
  const lastIndex = pages.length - 1;
  return options.getNextPageParam(
    pages[lastIndex],
    pages,
    pageParams[lastIndex],
    pageParams
  );
}
function getPreviousPageParam(options, { pages, pageParams }) {
  var _a12;
  return (_a12 = options.getPreviousPageParam) == null ? void 0 : _a12.call(
    options,
    pages[0],
    pages,
    pageParams[0],
    pageParams
  );
}
function hasNextPage(options, data) {
  if (!data)
    return false;
  return getNextPageParam(options, data) != null;
}
function hasPreviousPage(options, data) {
  if (!data || !options.getPreviousPageParam)
    return false;
  return getPreviousPageParam(options, data) != null;
}

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/queryClient.js
var _queryCache, _mutationCache2, _defaultOptions3, _queryDefaults, _mutationDefaults, _mountCount, _unsubscribeFocus, _unsubscribeOnline, _a8;
var QueryClient = (_a8 = class {
  constructor(config = {}) {
    __privateAdd(this, _queryCache, void 0);
    __privateAdd(this, _mutationCache2, void 0);
    __privateAdd(this, _defaultOptions3, void 0);
    __privateAdd(this, _queryDefaults, void 0);
    __privateAdd(this, _mutationDefaults, void 0);
    __privateAdd(this, _mountCount, void 0);
    __privateAdd(this, _unsubscribeFocus, void 0);
    __privateAdd(this, _unsubscribeOnline, void 0);
    __privateSet(this, _queryCache, config.queryCache || new QueryCache());
    __privateSet(this, _mutationCache2, config.mutationCache || new MutationCache());
    __privateSet(this, _defaultOptions3, config.defaultOptions || {});
    __privateSet(this, _queryDefaults, /* @__PURE__ */ new Map());
    __privateSet(this, _mutationDefaults, /* @__PURE__ */ new Map());
    __privateSet(this, _mountCount, 0);
  }
  mount() {
    __privateWrapper(this, _mountCount)._++;
    if (__privateGet(this, _mountCount) !== 1)
      return;
    __privateSet(this, _unsubscribeFocus, focusManager.subscribe(() => {
      if (focusManager.isFocused()) {
        this.resumePausedMutations();
        __privateGet(this, _queryCache).onFocus();
      }
    }));
    __privateSet(this, _unsubscribeOnline, onlineManager.subscribe(() => {
      if (onlineManager.isOnline()) {
        this.resumePausedMutations();
        __privateGet(this, _queryCache).onOnline();
      }
    }));
  }
  unmount() {
    var _a12, _b;
    __privateWrapper(this, _mountCount)._--;
    if (__privateGet(this, _mountCount) !== 0)
      return;
    (_a12 = __privateGet(this, _unsubscribeFocus)) == null ? void 0 : _a12.call(this);
    __privateSet(this, _unsubscribeFocus, void 0);
    (_b = __privateGet(this, _unsubscribeOnline)) == null ? void 0 : _b.call(this);
    __privateSet(this, _unsubscribeOnline, void 0);
  }
  isFetching(filters) {
    return __privateGet(this, _queryCache).findAll({ ...filters, fetchStatus: "fetching" }).length;
  }
  isMutating(filters) {
    return __privateGet(this, _mutationCache2).findAll({ ...filters, status: "pending" }).length;
  }
  getQueryData(queryKey) {
    var _a12;
    return (_a12 = __privateGet(this, _queryCache).find({ queryKey })) == null ? void 0 : _a12.state.data;
  }
  ensureQueryData(options) {
    const cachedData = this.getQueryData(options.queryKey);
    return cachedData !== void 0 ? Promise.resolve(cachedData) : this.fetchQuery(options);
  }
  getQueriesData(filters) {
    return this.getQueryCache().findAll(filters).map(({ queryKey, state }) => {
      const data = state.data;
      return [queryKey, data];
    });
  }
  setQueryData(queryKey, updater, options) {
    const query = __privateGet(this, _queryCache).find({ queryKey });
    const prevData = query == null ? void 0 : query.state.data;
    const data = functionalUpdate(updater, prevData);
    if (typeof data === "undefined") {
      return void 0;
    }
    const defaultedOptions = this.defaultQueryOptions({ queryKey });
    return __privateGet(this, _queryCache).build(this, defaultedOptions).setData(data, { ...options, manual: true });
  }
  setQueriesData(filters, updater, options) {
    return notifyManager.batch(
      () => this.getQueryCache().findAll(filters).map(({ queryKey }) => [
        queryKey,
        this.setQueryData(queryKey, updater, options)
      ])
    );
  }
  getQueryState(queryKey) {
    var _a12;
    return (_a12 = __privateGet(this, _queryCache).find({ queryKey })) == null ? void 0 : _a12.state;
  }
  removeQueries(filters) {
    const queryCache = __privateGet(this, _queryCache);
    notifyManager.batch(() => {
      queryCache.findAll(filters).forEach((query) => {
        queryCache.remove(query);
      });
    });
  }
  resetQueries(filters, options) {
    const queryCache = __privateGet(this, _queryCache);
    const refetchFilters = {
      type: "active",
      ...filters
    };
    return notifyManager.batch(() => {
      queryCache.findAll(filters).forEach((query) => {
        query.reset();
      });
      return this.refetchQueries(refetchFilters, options);
    });
  }
  cancelQueries(filters = {}, cancelOptions = {}) {
    const defaultedCancelOptions = { revert: true, ...cancelOptions };
    const promises = notifyManager.batch(
      () => __privateGet(this, _queryCache).findAll(filters).map((query) => query.cancel(defaultedCancelOptions))
    );
    return Promise.all(promises).then(noop).catch(noop);
  }
  invalidateQueries(filters = {}, options = {}) {
    return notifyManager.batch(() => {
      __privateGet(this, _queryCache).findAll(filters).forEach((query) => {
        query.invalidate();
      });
      if (filters.refetchType === "none") {
        return Promise.resolve();
      }
      const refetchFilters = {
        ...filters,
        type: filters.refetchType ?? filters.type ?? "active"
      };
      return this.refetchQueries(refetchFilters, options);
    });
  }
  refetchQueries(filters = {}, options) {
    const fetchOptions = {
      ...options,
      cancelRefetch: (options == null ? void 0 : options.cancelRefetch) ?? true
    };
    const promises = notifyManager.batch(
      () => __privateGet(this, _queryCache).findAll(filters).filter((query) => !query.isDisabled()).map((query) => {
        let promise = query.fetch(void 0, fetchOptions);
        if (!fetchOptions.throwOnError) {
          promise = promise.catch(noop);
        }
        return query.state.fetchStatus === "paused" ? Promise.resolve() : promise;
      })
    );
    return Promise.all(promises).then(noop);
  }
  fetchQuery(options) {
    const defaultedOptions = this.defaultQueryOptions(options);
    if (typeof defaultedOptions.retry === "undefined") {
      defaultedOptions.retry = false;
    }
    const query = __privateGet(this, _queryCache).build(this, defaultedOptions);
    return query.isStaleByTime(defaultedOptions.staleTime) ? query.fetch(defaultedOptions) : Promise.resolve(query.state.data);
  }
  prefetchQuery(options) {
    return this.fetchQuery(options).then(noop).catch(noop);
  }
  fetchInfiniteQuery(options) {
    options.behavior = infiniteQueryBehavior(options.pages);
    return this.fetchQuery(options);
  }
  prefetchInfiniteQuery(options) {
    return this.fetchInfiniteQuery(options).then(noop).catch(noop);
  }
  resumePausedMutations() {
    return __privateGet(this, _mutationCache2).resumePausedMutations();
  }
  getQueryCache() {
    return __privateGet(this, _queryCache);
  }
  getMutationCache() {
    return __privateGet(this, _mutationCache2);
  }
  getDefaultOptions() {
    return __privateGet(this, _defaultOptions3);
  }
  setDefaultOptions(options) {
    __privateSet(this, _defaultOptions3, options);
  }
  setQueryDefaults(queryKey, options) {
    __privateGet(this, _queryDefaults).set(hashKey(queryKey), {
      queryKey,
      defaultOptions: options
    });
  }
  getQueryDefaults(queryKey) {
    const defaults = [...__privateGet(this, _queryDefaults).values()];
    let result = {};
    defaults.forEach((queryDefault) => {
      if (partialMatchKey(queryKey, queryDefault.queryKey)) {
        result = { ...result, ...queryDefault.defaultOptions };
      }
    });
    return result;
  }
  setMutationDefaults(mutationKey, options) {
    __privateGet(this, _mutationDefaults).set(hashKey(mutationKey), {
      mutationKey,
      defaultOptions: options
    });
  }
  getMutationDefaults(mutationKey) {
    const defaults = [...__privateGet(this, _mutationDefaults).values()];
    let result = {};
    defaults.forEach((queryDefault) => {
      if (partialMatchKey(mutationKey, queryDefault.mutationKey)) {
        result = { ...result, ...queryDefault.defaultOptions };
      }
    });
    return result;
  }
  defaultQueryOptions(options) {
    if (options == null ? void 0 : options._defaulted) {
      return options;
    }
    const defaultedOptions = {
      ...__privateGet(this, _defaultOptions3).queries,
      ...(options == null ? void 0 : options.queryKey) && this.getQueryDefaults(options.queryKey),
      ...options,
      _defaulted: true
    };
    if (!defaultedOptions.queryHash) {
      defaultedOptions.queryHash = hashQueryKeyByOptions(
        defaultedOptions.queryKey,
        defaultedOptions
      );
    }
    if (typeof defaultedOptions.refetchOnReconnect === "undefined") {
      defaultedOptions.refetchOnReconnect = defaultedOptions.networkMode !== "always";
    }
    if (typeof defaultedOptions.throwOnError === "undefined") {
      defaultedOptions.throwOnError = !!defaultedOptions.suspense;
    }
    if (typeof defaultedOptions.networkMode === "undefined" && defaultedOptions.persister) {
      defaultedOptions.networkMode = "offlineFirst";
    }
    return defaultedOptions;
  }
  defaultMutationOptions(options) {
    if (options == null ? void 0 : options._defaulted) {
      return options;
    }
    return {
      ...__privateGet(this, _defaultOptions3).mutations,
      ...(options == null ? void 0 : options.mutationKey) && this.getMutationDefaults(options.mutationKey),
      ...options,
      _defaulted: true
    };
  }
  clear() {
    __privateGet(this, _queryCache).clear();
    __privateGet(this, _mutationCache2).clear();
  }
}, _queryCache = new WeakMap(), _mutationCache2 = new WeakMap(), _defaultOptions3 = new WeakMap(), _queryDefaults = new WeakMap(), _mutationDefaults = new WeakMap(), _mountCount = new WeakMap(), _unsubscribeFocus = new WeakMap(), _unsubscribeOnline = new WeakMap(), _a8);

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/queryObserver.js
var _client, _currentQuery, _currentQueryInitialState, _currentResult, _currentResultState, _currentResultOptions, _selectError, _selectFn, _selectResult, _lastQueryWithDefinedData, _staleTimeoutId, _refetchIntervalId, _currentRefetchInterval, _trackedProps, _executeFetch, executeFetch_fn, _updateStaleTimeout, updateStaleTimeout_fn, _computeRefetchInterval, computeRefetchInterval_fn, _updateRefetchInterval, updateRefetchInterval_fn, _updateTimers, updateTimers_fn, _clearStaleTimeout, clearStaleTimeout_fn, _clearRefetchInterval, clearRefetchInterval_fn, _updateQuery, updateQuery_fn, _notify, notify_fn, _a9;
var QueryObserver = (_a9 = class extends Subscribable {
  constructor(client, options) {
    super();
    __privateAdd(this, _executeFetch);
    __privateAdd(this, _updateStaleTimeout);
    __privateAdd(this, _computeRefetchInterval);
    __privateAdd(this, _updateRefetchInterval);
    __privateAdd(this, _updateTimers);
    __privateAdd(this, _clearStaleTimeout);
    __privateAdd(this, _clearRefetchInterval);
    __privateAdd(this, _updateQuery);
    __privateAdd(this, _notify);
    __privateAdd(this, _client, void 0);
    __privateAdd(this, _currentQuery, void 0);
    __privateAdd(this, _currentQueryInitialState, void 0);
    __privateAdd(this, _currentResult, void 0);
    __privateAdd(this, _currentResultState, void 0);
    __privateAdd(this, _currentResultOptions, void 0);
    __privateAdd(this, _selectError, void 0);
    __privateAdd(this, _selectFn, void 0);
    __privateAdd(this, _selectResult, void 0);
    // This property keeps track of the last query with defined data.
    // It will be used to pass the previous data and query to the placeholder function between renders.
    __privateAdd(this, _lastQueryWithDefinedData, void 0);
    __privateAdd(this, _staleTimeoutId, void 0);
    __privateAdd(this, _refetchIntervalId, void 0);
    __privateAdd(this, _currentRefetchInterval, void 0);
    __privateAdd(this, _trackedProps, /* @__PURE__ */ new Set());
    this.options = options;
    __privateSet(this, _client, client);
    __privateSet(this, _selectError, null);
    this.bindMethods();
    this.setOptions(options);
  }
  bindMethods() {
    this.refetch = this.refetch.bind(this);
  }
  onSubscribe() {
    if (this.listeners.size === 1) {
      __privateGet(this, _currentQuery).addObserver(this);
      if (shouldFetchOnMount(__privateGet(this, _currentQuery), this.options)) {
        __privateMethod(this, _executeFetch, executeFetch_fn).call(this);
      } else {
        this.updateResult();
      }
      __privateMethod(this, _updateTimers, updateTimers_fn).call(this);
    }
  }
  onUnsubscribe() {
    if (!this.hasListeners()) {
      this.destroy();
    }
  }
  shouldFetchOnReconnect() {
    return shouldFetchOn(
      __privateGet(this, _currentQuery),
      this.options,
      this.options.refetchOnReconnect
    );
  }
  shouldFetchOnWindowFocus() {
    return shouldFetchOn(
      __privateGet(this, _currentQuery),
      this.options,
      this.options.refetchOnWindowFocus
    );
  }
  destroy() {
    this.listeners = /* @__PURE__ */ new Set();
    __privateMethod(this, _clearStaleTimeout, clearStaleTimeout_fn).call(this);
    __privateMethod(this, _clearRefetchInterval, clearRefetchInterval_fn).call(this);
    __privateGet(this, _currentQuery).removeObserver(this);
  }
  setOptions(options, notifyOptions) {
    const prevOptions = this.options;
    const prevQuery = __privateGet(this, _currentQuery);
    this.options = __privateGet(this, _client).defaultQueryOptions(options);
    if (!shallowEqualObjects(prevOptions, this.options)) {
      __privateGet(this, _client).getQueryCache().notify({
        type: "observerOptionsUpdated",
        query: __privateGet(this, _currentQuery),
        observer: this
      });
    }
    if (typeof this.options.enabled !== "undefined" && typeof this.options.enabled !== "boolean") {
      throw new Error("Expected enabled to be a boolean");
    }
    if (!this.options.queryKey) {
      this.options.queryKey = prevOptions.queryKey;
    }
    __privateMethod(this, _updateQuery, updateQuery_fn).call(this);
    const mounted = this.hasListeners();
    if (mounted && shouldFetchOptionally(
      __privateGet(this, _currentQuery),
      prevQuery,
      this.options,
      prevOptions
    )) {
      __privateMethod(this, _executeFetch, executeFetch_fn).call(this);
    }
    this.updateResult(notifyOptions);
    if (mounted && (__privateGet(this, _currentQuery) !== prevQuery || this.options.enabled !== prevOptions.enabled || this.options.staleTime !== prevOptions.staleTime)) {
      __privateMethod(this, _updateStaleTimeout, updateStaleTimeout_fn).call(this);
    }
    const nextRefetchInterval = __privateMethod(this, _computeRefetchInterval, computeRefetchInterval_fn).call(this);
    if (mounted && (__privateGet(this, _currentQuery) !== prevQuery || this.options.enabled !== prevOptions.enabled || nextRefetchInterval !== __privateGet(this, _currentRefetchInterval))) {
      __privateMethod(this, _updateRefetchInterval, updateRefetchInterval_fn).call(this, nextRefetchInterval);
    }
  }
  getOptimisticResult(options) {
    const query = __privateGet(this, _client).getQueryCache().build(__privateGet(this, _client), options);
    const result = this.createResult(query, options);
    if (shouldAssignObserverCurrentProperties(this, result)) {
      __privateSet(this, _currentResult, result);
      __privateSet(this, _currentResultOptions, this.options);
      __privateSet(this, _currentResultState, __privateGet(this, _currentQuery).state);
    }
    return result;
  }
  getCurrentResult() {
    return __privateGet(this, _currentResult);
  }
  trackResult(result) {
    const trackedResult = {};
    Object.keys(result).forEach((key) => {
      Object.defineProperty(trackedResult, key, {
        configurable: false,
        enumerable: true,
        get: () => {
          __privateGet(this, _trackedProps).add(key);
          return result[key];
        }
      });
    });
    return trackedResult;
  }
  getCurrentQuery() {
    return __privateGet(this, _currentQuery);
  }
  refetch({ ...options } = {}) {
    return this.fetch({
      ...options
    });
  }
  fetchOptimistic(options) {
    const defaultedOptions = __privateGet(this, _client).defaultQueryOptions(options);
    const query = __privateGet(this, _client).getQueryCache().build(__privateGet(this, _client), defaultedOptions);
    query.isFetchingOptimistic = true;
    return query.fetch().then(() => this.createResult(query, defaultedOptions));
  }
  fetch(fetchOptions) {
    return __privateMethod(this, _executeFetch, executeFetch_fn).call(this, {
      ...fetchOptions,
      cancelRefetch: fetchOptions.cancelRefetch ?? true
    }).then(() => {
      this.updateResult();
      return __privateGet(this, _currentResult);
    });
  }
  createResult(query, options) {
    var _a12;
    const prevQuery = __privateGet(this, _currentQuery);
    const prevOptions = this.options;
    const prevResult = __privateGet(this, _currentResult);
    const prevResultState = __privateGet(this, _currentResultState);
    const prevResultOptions = __privateGet(this, _currentResultOptions);
    const queryChange = query !== prevQuery;
    const queryInitialState = queryChange ? query.state : __privateGet(this, _currentQueryInitialState);
    const { state } = query;
    let { error, errorUpdatedAt, fetchStatus, status } = state;
    let isPlaceholderData = false;
    let data;
    if (options._optimisticResults) {
      const mounted = this.hasListeners();
      const fetchOnMount = !mounted && shouldFetchOnMount(query, options);
      const fetchOptionally = mounted && shouldFetchOptionally(query, prevQuery, options, prevOptions);
      if (fetchOnMount || fetchOptionally) {
        fetchStatus = canFetch(query.options.networkMode) ? "fetching" : "paused";
        if (!state.dataUpdatedAt) {
          status = "pending";
        }
      }
      if (options._optimisticResults === "isRestoring") {
        fetchStatus = "idle";
      }
    }
    if (options.select && typeof state.data !== "undefined") {
      if (prevResult && state.data === (prevResultState == null ? void 0 : prevResultState.data) && options.select === __privateGet(this, _selectFn)) {
        data = __privateGet(this, _selectResult);
      } else {
        try {
          __privateSet(this, _selectFn, options.select);
          data = options.select(state.data);
          data = replaceData(prevResult == null ? void 0 : prevResult.data, data, options);
          __privateSet(this, _selectResult, data);
          __privateSet(this, _selectError, null);
        } catch (selectError) {
          __privateSet(this, _selectError, selectError);
        }
      }
    } else {
      data = state.data;
    }
    if (typeof options.placeholderData !== "undefined" && typeof data === "undefined" && status === "pending") {
      let placeholderData;
      if ((prevResult == null ? void 0 : prevResult.isPlaceholderData) && options.placeholderData === (prevResultOptions == null ? void 0 : prevResultOptions.placeholderData)) {
        placeholderData = prevResult.data;
      } else {
        placeholderData = typeof options.placeholderData === "function" ? options.placeholderData(
          (_a12 = __privateGet(this, _lastQueryWithDefinedData)) == null ? void 0 : _a12.state.data,
          __privateGet(this, _lastQueryWithDefinedData)
        ) : options.placeholderData;
        if (options.select && typeof placeholderData !== "undefined") {
          try {
            placeholderData = options.select(placeholderData);
            __privateSet(this, _selectError, null);
          } catch (selectError) {
            __privateSet(this, _selectError, selectError);
          }
        }
      }
      if (typeof placeholderData !== "undefined") {
        status = "success";
        data = replaceData(
          prevResult == null ? void 0 : prevResult.data,
          placeholderData,
          options
        );
        isPlaceholderData = true;
      }
    }
    if (__privateGet(this, _selectError)) {
      error = __privateGet(this, _selectError);
      data = __privateGet(this, _selectResult);
      errorUpdatedAt = Date.now();
      status = "error";
    }
    const isFetching = fetchStatus === "fetching";
    const isPending = status === "pending";
    const isError = status === "error";
    const isLoading = isPending && isFetching;
    const result = {
      status,
      fetchStatus,
      isPending,
      isSuccess: status === "success",
      isError,
      isInitialLoading: isLoading,
      isLoading,
      data,
      dataUpdatedAt: state.dataUpdatedAt,
      error,
      errorUpdatedAt,
      failureCount: state.fetchFailureCount,
      failureReason: state.fetchFailureReason,
      errorUpdateCount: state.errorUpdateCount,
      isFetched: state.dataUpdateCount > 0 || state.errorUpdateCount > 0,
      isFetchedAfterMount: state.dataUpdateCount > queryInitialState.dataUpdateCount || state.errorUpdateCount > queryInitialState.errorUpdateCount,
      isFetching,
      isRefetching: isFetching && !isPending,
      isLoadingError: isError && state.dataUpdatedAt === 0,
      isPaused: fetchStatus === "paused",
      isPlaceholderData,
      isRefetchError: isError && state.dataUpdatedAt !== 0,
      isStale: isStale(query, options),
      refetch: this.refetch
    };
    return result;
  }
  updateResult(notifyOptions) {
    const prevResult = __privateGet(this, _currentResult);
    const nextResult = this.createResult(__privateGet(this, _currentQuery), this.options);
    __privateSet(this, _currentResultState, __privateGet(this, _currentQuery).state);
    __privateSet(this, _currentResultOptions, this.options);
    if (__privateGet(this, _currentResultState).data !== void 0) {
      __privateSet(this, _lastQueryWithDefinedData, __privateGet(this, _currentQuery));
    }
    if (shallowEqualObjects(nextResult, prevResult)) {
      return;
    }
    __privateSet(this, _currentResult, nextResult);
    const defaultNotifyOptions = {};
    const shouldNotifyListeners = () => {
      if (!prevResult) {
        return true;
      }
      const { notifyOnChangeProps } = this.options;
      const notifyOnChangePropsValue = typeof notifyOnChangeProps === "function" ? notifyOnChangeProps() : notifyOnChangeProps;
      if (notifyOnChangePropsValue === "all" || !notifyOnChangePropsValue && !__privateGet(this, _trackedProps).size) {
        return true;
      }
      const includedProps = new Set(
        notifyOnChangePropsValue ?? __privateGet(this, _trackedProps)
      );
      if (this.options.throwOnError) {
        includedProps.add("error");
      }
      return Object.keys(__privateGet(this, _currentResult)).some((key) => {
        const typedKey = key;
        const changed = __privateGet(this, _currentResult)[typedKey] !== prevResult[typedKey];
        return changed && includedProps.has(typedKey);
      });
    };
    if ((notifyOptions == null ? void 0 : notifyOptions.listeners) !== false && shouldNotifyListeners()) {
      defaultNotifyOptions.listeners = true;
    }
    __privateMethod(this, _notify, notify_fn).call(this, { ...defaultNotifyOptions, ...notifyOptions });
  }
  onQueryUpdate() {
    this.updateResult();
    if (this.hasListeners()) {
      __privateMethod(this, _updateTimers, updateTimers_fn).call(this);
    }
  }
}, _client = new WeakMap(), _currentQuery = new WeakMap(), _currentQueryInitialState = new WeakMap(), _currentResult = new WeakMap(), _currentResultState = new WeakMap(), _currentResultOptions = new WeakMap(), _selectError = new WeakMap(), _selectFn = new WeakMap(), _selectResult = new WeakMap(), _lastQueryWithDefinedData = new WeakMap(), _staleTimeoutId = new WeakMap(), _refetchIntervalId = new WeakMap(), _currentRefetchInterval = new WeakMap(), _trackedProps = new WeakMap(), _executeFetch = new WeakSet(), executeFetch_fn = function(fetchOptions) {
  __privateMethod(this, _updateQuery, updateQuery_fn).call(this);
  let promise = __privateGet(this, _currentQuery).fetch(
    this.options,
    fetchOptions
  );
  if (!(fetchOptions == null ? void 0 : fetchOptions.throwOnError)) {
    promise = promise.catch(noop);
  }
  return promise;
}, _updateStaleTimeout = new WeakSet(), updateStaleTimeout_fn = function() {
  __privateMethod(this, _clearStaleTimeout, clearStaleTimeout_fn).call(this);
  if (isServer || __privateGet(this, _currentResult).isStale || !isValidTimeout(this.options.staleTime)) {
    return;
  }
  const time = timeUntilStale(
    __privateGet(this, _currentResult).dataUpdatedAt,
    this.options.staleTime
  );
  const timeout = time + 1;
  __privateSet(this, _staleTimeoutId, setTimeout(() => {
    if (!__privateGet(this, _currentResult).isStale) {
      this.updateResult();
    }
  }, timeout));
}, _computeRefetchInterval = new WeakSet(), computeRefetchInterval_fn = function() {
  return (typeof this.options.refetchInterval === "function" ? this.options.refetchInterval(__privateGet(this, _currentQuery)) : this.options.refetchInterval) ?? false;
}, _updateRefetchInterval = new WeakSet(), updateRefetchInterval_fn = function(nextInterval) {
  __privateMethod(this, _clearRefetchInterval, clearRefetchInterval_fn).call(this);
  __privateSet(this, _currentRefetchInterval, nextInterval);
  if (isServer || this.options.enabled === false || !isValidTimeout(__privateGet(this, _currentRefetchInterval)) || __privateGet(this, _currentRefetchInterval) === 0) {
    return;
  }
  __privateSet(this, _refetchIntervalId, setInterval(() => {
    if (this.options.refetchIntervalInBackground || focusManager.isFocused()) {
      __privateMethod(this, _executeFetch, executeFetch_fn).call(this);
    }
  }, __privateGet(this, _currentRefetchInterval)));
}, _updateTimers = new WeakSet(), updateTimers_fn = function() {
  __privateMethod(this, _updateStaleTimeout, updateStaleTimeout_fn).call(this);
  __privateMethod(this, _updateRefetchInterval, updateRefetchInterval_fn).call(this, __privateMethod(this, _computeRefetchInterval, computeRefetchInterval_fn).call(this));
}, _clearStaleTimeout = new WeakSet(), clearStaleTimeout_fn = function() {
  if (__privateGet(this, _staleTimeoutId)) {
    clearTimeout(__privateGet(this, _staleTimeoutId));
    __privateSet(this, _staleTimeoutId, void 0);
  }
}, _clearRefetchInterval = new WeakSet(), clearRefetchInterval_fn = function() {
  if (__privateGet(this, _refetchIntervalId)) {
    clearInterval(__privateGet(this, _refetchIntervalId));
    __privateSet(this, _refetchIntervalId, void 0);
  }
}, _updateQuery = new WeakSet(), updateQuery_fn = function() {
  const query = __privateGet(this, _client).getQueryCache().build(__privateGet(this, _client), this.options);
  if (query === __privateGet(this, _currentQuery)) {
    return;
  }
  const prevQuery = __privateGet(this, _currentQuery);
  __privateSet(this, _currentQuery, query);
  __privateSet(this, _currentQueryInitialState, query.state);
  if (this.hasListeners()) {
    prevQuery == null ? void 0 : prevQuery.removeObserver(this);
    query.addObserver(this);
  }
}, _notify = new WeakSet(), notify_fn = function(notifyOptions) {
  notifyManager.batch(() => {
    if (notifyOptions.listeners) {
      this.listeners.forEach((listener) => {
        listener(__privateGet(this, _currentResult));
      });
    }
    __privateGet(this, _client).getQueryCache().notify({
      query: __privateGet(this, _currentQuery),
      type: "observerResultsUpdated"
    });
  });
}, _a9);
function shouldLoadOnMount(query, options) {
  return options.enabled !== false && !query.state.dataUpdatedAt && !(query.state.status === "error" && options.retryOnMount === false);
}
function shouldFetchOnMount(query, options) {
  return shouldLoadOnMount(query, options) || query.state.dataUpdatedAt > 0 && shouldFetchOn(query, options, options.refetchOnMount);
}
function shouldFetchOn(query, options, field) {
  if (options.enabled !== false) {
    const value = typeof field === "function" ? field(query) : field;
    return value === "always" || value !== false && isStale(query, options);
  }
  return false;
}
function shouldFetchOptionally(query, prevQuery, options, prevOptions) {
  return options.enabled !== false && (query !== prevQuery || prevOptions.enabled === false) && (!options.suspense || query.state.status !== "error") && isStale(query, options);
}
function isStale(query, options) {
  return query.isStaleByTime(options.staleTime);
}
function shouldAssignObserverCurrentProperties(observer, optimisticResult) {
  if (!shallowEqualObjects(observer.getCurrentResult(), optimisticResult)) {
    return true;
  }
  return false;
}

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/queriesObserver.js
function difference(array1, array2) {
  return array1.filter((x) => !array2.includes(x));
}
function replaceAt(array, index, value) {
  const copy = array.slice(0);
  copy[index] = value;
  return copy;
}
var _client2, _result, _queries2, _observers3, _options, _combinedResult, _setResult, setResult_fn, _combineResult, combineResult_fn, _findMatchingObservers, findMatchingObservers_fn, _onUpdate, onUpdate_fn, _notify2, notify_fn2, _a10;
var QueriesObserver = (_a10 = class extends Subscribable {
  constructor(client, queries, options) {
    super();
    __privateAdd(this, _setResult);
    __privateAdd(this, _combineResult);
    __privateAdd(this, _findMatchingObservers);
    __privateAdd(this, _onUpdate);
    __privateAdd(this, _notify2);
    __privateAdd(this, _client2, void 0);
    __privateAdd(this, _result, void 0);
    __privateAdd(this, _queries2, void 0);
    __privateAdd(this, _observers3, void 0);
    __privateAdd(this, _options, void 0);
    __privateAdd(this, _combinedResult, void 0);
    __privateSet(this, _client2, client);
    __privateSet(this, _queries2, []);
    __privateSet(this, _observers3, []);
    __privateMethod(this, _setResult, setResult_fn).call(this, []);
    this.setQueries(queries, options);
  }
  onSubscribe() {
    if (this.listeners.size === 1) {
      __privateGet(this, _observers3).forEach((observer) => {
        observer.subscribe((result) => {
          __privateMethod(this, _onUpdate, onUpdate_fn).call(this, observer, result);
        });
      });
    }
  }
  onUnsubscribe() {
    if (!this.listeners.size) {
      this.destroy();
    }
  }
  destroy() {
    this.listeners = /* @__PURE__ */ new Set();
    __privateGet(this, _observers3).forEach((observer) => {
      observer.destroy();
    });
  }
  setQueries(queries, options, notifyOptions) {
    __privateSet(this, _queries2, queries);
    __privateSet(this, _options, options);
    notifyManager.batch(() => {
      const prevObservers = __privateGet(this, _observers3);
      const newObserverMatches = __privateMethod(this, _findMatchingObservers, findMatchingObservers_fn).call(this, __privateGet(this, _queries2));
      newObserverMatches.forEach(
        (match) => match.observer.setOptions(match.defaultedQueryOptions, notifyOptions)
      );
      const newObservers = newObserverMatches.map((match) => match.observer);
      const newResult = newObservers.map(
        (observer) => observer.getCurrentResult()
      );
      const hasIndexChange = newObservers.some(
        (observer, index) => observer !== prevObservers[index]
      );
      if (prevObservers.length === newObservers.length && !hasIndexChange) {
        return;
      }
      __privateSet(this, _observers3, newObservers);
      __privateMethod(this, _setResult, setResult_fn).call(this, newResult);
      if (!this.hasListeners()) {
        return;
      }
      difference(prevObservers, newObservers).forEach((observer) => {
        observer.destroy();
      });
      difference(newObservers, prevObservers).forEach((observer) => {
        observer.subscribe((result) => {
          __privateMethod(this, _onUpdate, onUpdate_fn).call(this, observer, result);
        });
      });
      __privateMethod(this, _notify2, notify_fn2).call(this);
    });
  }
  getCurrentResult() {
    return __privateGet(this, _combinedResult);
  }
  getQueries() {
    return __privateGet(this, _observers3).map((observer) => observer.getCurrentQuery());
  }
  getObservers() {
    return __privateGet(this, _observers3);
  }
  getOptimisticResult(queries, combine) {
    const matches = __privateMethod(this, _findMatchingObservers, findMatchingObservers_fn).call(this, queries);
    const result = matches.map(
      (match) => match.observer.getOptimisticResult(match.defaultedQueryOptions)
    );
    return [
      result,
      (r) => {
        return __privateMethod(this, _combineResult, combineResult_fn).call(this, r ?? result, combine);
      },
      () => {
        return matches.map((match, index) => {
          const observerResult = result[index];
          return !match.defaultedQueryOptions.notifyOnChangeProps ? match.observer.trackResult(observerResult) : observerResult;
        });
      }
    ];
  }
}, _client2 = new WeakMap(), _result = new WeakMap(), _queries2 = new WeakMap(), _observers3 = new WeakMap(), _options = new WeakMap(), _combinedResult = new WeakMap(), _setResult = new WeakSet(), setResult_fn = function(value) {
  var _a12;
  __privateSet(this, _result, value);
  __privateSet(this, _combinedResult, __privateMethod(this, _combineResult, combineResult_fn).call(this, value, (_a12 = __privateGet(this, _options)) == null ? void 0 : _a12.combine));
}, _combineResult = new WeakSet(), combineResult_fn = function(input, combine) {
  if (combine) {
    return replaceEqualDeep(__privateGet(this, _combinedResult), combine(input));
  }
  return input;
}, _findMatchingObservers = new WeakSet(), findMatchingObservers_fn = function(queries) {
  const prevObservers = __privateGet(this, _observers3);
  const prevObserversMap = new Map(
    prevObservers.map((observer) => [observer.options.queryHash, observer])
  );
  const defaultedQueryOptions = queries.map(
    (options) => __privateGet(this, _client2).defaultQueryOptions(options)
  );
  const matchingObservers = defaultedQueryOptions.flatMap((defaultedOptions) => {
    const match = prevObserversMap.get(defaultedOptions.queryHash);
    if (match != null) {
      return [{ defaultedQueryOptions: defaultedOptions, observer: match }];
    }
    return [];
  });
  const matchedQueryHashes = new Set(
    matchingObservers.map((match) => match.defaultedQueryOptions.queryHash)
  );
  const unmatchedQueries = defaultedQueryOptions.filter(
    (defaultedOptions) => !matchedQueryHashes.has(defaultedOptions.queryHash)
  );
  const getObserver = (options) => {
    const defaultedOptions = __privateGet(this, _client2).defaultQueryOptions(options);
    const currentObserver = __privateGet(this, _observers3).find(
      (o) => o.options.queryHash === defaultedOptions.queryHash
    );
    return currentObserver ?? new QueryObserver(__privateGet(this, _client2), defaultedOptions);
  };
  const newOrReusedObservers = unmatchedQueries.map((options) => {
    return {
      defaultedQueryOptions: options,
      observer: getObserver(options)
    };
  });
  const sortMatchesByOrderOfQueries = (a, b) => defaultedQueryOptions.indexOf(a.defaultedQueryOptions) - defaultedQueryOptions.indexOf(b.defaultedQueryOptions);
  return matchingObservers.concat(newOrReusedObservers).sort(sortMatchesByOrderOfQueries);
}, _onUpdate = new WeakSet(), onUpdate_fn = function(observer, result) {
  const index = __privateGet(this, _observers3).indexOf(observer);
  if (index !== -1) {
    __privateMethod(this, _setResult, setResult_fn).call(this, replaceAt(__privateGet(this, _result), index, result));
    __privateMethod(this, _notify2, notify_fn2).call(this);
  }
}, _notify2 = new WeakSet(), notify_fn2 = function() {
  notifyManager.batch(() => {
    this.listeners.forEach((listener) => {
      listener(__privateGet(this, _result));
    });
  });
}, _a10);

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/infiniteQueryObserver.js
var InfiniteQueryObserver = class extends QueryObserver {
  // eslint-disable-next-line @typescript-eslint/no-useless-constructor
  constructor(client, options) {
    super(client, options);
  }
  bindMethods() {
    super.bindMethods();
    this.fetchNextPage = this.fetchNextPage.bind(this);
    this.fetchPreviousPage = this.fetchPreviousPage.bind(this);
  }
  setOptions(options, notifyOptions) {
    super.setOptions(
      {
        ...options,
        behavior: infiniteQueryBehavior()
      },
      notifyOptions
    );
  }
  getOptimisticResult(options) {
    options.behavior = infiniteQueryBehavior();
    return super.getOptimisticResult(options);
  }
  fetchNextPage(options) {
    return this.fetch({
      ...options,
      meta: {
        fetchMore: { direction: "forward" }
      }
    });
  }
  fetchPreviousPage(options) {
    return this.fetch({
      ...options,
      meta: {
        fetchMore: { direction: "backward" }
      }
    });
  }
  createResult(query, options) {
    var _a12, _b, _c, _d;
    const { state } = query;
    const result = super.createResult(query, options);
    const { isFetching, isRefetching } = result;
    const isFetchingNextPage = isFetching && ((_b = (_a12 = state.fetchMeta) == null ? void 0 : _a12.fetchMore) == null ? void 0 : _b.direction) === "forward";
    const isFetchingPreviousPage = isFetching && ((_d = (_c = state.fetchMeta) == null ? void 0 : _c.fetchMore) == null ? void 0 : _d.direction) === "backward";
    return {
      ...result,
      fetchNextPage: this.fetchNextPage,
      fetchPreviousPage: this.fetchPreviousPage,
      hasNextPage: hasNextPage(options, state.data),
      hasPreviousPage: hasPreviousPage(options, state.data),
      isFetchingNextPage,
      isFetchingPreviousPage,
      isRefetching: isRefetching && !isFetchingNextPage && !isFetchingPreviousPage
    };
  }
};

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/mutationObserver.js
var _client3, _currentResult2, _currentMutation, _mutateOptions, _updateResult, updateResult_fn, _notify3, notify_fn3, _a11;
var MutationObserver = (_a11 = class extends Subscribable {
  constructor(client, options) {
    super();
    __privateAdd(this, _updateResult);
    __privateAdd(this, _notify3);
    __privateAdd(this, _client3, void 0);
    __privateAdd(this, _currentResult2, void 0);
    __privateAdd(this, _currentMutation, void 0);
    __privateAdd(this, _mutateOptions, void 0);
    __privateSet(this, _currentResult2, void 0);
    __privateSet(this, _client3, client);
    this.setOptions(options);
    this.bindMethods();
    __privateMethod(this, _updateResult, updateResult_fn).call(this);
  }
  bindMethods() {
    this.mutate = this.mutate.bind(this);
    this.reset = this.reset.bind(this);
  }
  setOptions(options) {
    var _a12;
    const prevOptions = this.options;
    this.options = __privateGet(this, _client3).defaultMutationOptions(options);
    if (!shallowEqualObjects(prevOptions, this.options)) {
      __privateGet(this, _client3).getMutationCache().notify({
        type: "observerOptionsUpdated",
        mutation: __privateGet(this, _currentMutation),
        observer: this
      });
    }
    (_a12 = __privateGet(this, _currentMutation)) == null ? void 0 : _a12.setOptions(this.options);
    if ((prevOptions == null ? void 0 : prevOptions.mutationKey) && this.options.mutationKey && hashKey(prevOptions.mutationKey) !== hashKey(this.options.mutationKey)) {
      this.reset();
    }
  }
  onUnsubscribe() {
    var _a12;
    if (!this.hasListeners()) {
      (_a12 = __privateGet(this, _currentMutation)) == null ? void 0 : _a12.removeObserver(this);
    }
  }
  onMutationUpdate(action) {
    __privateMethod(this, _updateResult, updateResult_fn).call(this);
    __privateMethod(this, _notify3, notify_fn3).call(this, action);
  }
  getCurrentResult() {
    return __privateGet(this, _currentResult2);
  }
  reset() {
    var _a12;
    (_a12 = __privateGet(this, _currentMutation)) == null ? void 0 : _a12.removeObserver(this);
    __privateSet(this, _currentMutation, void 0);
    __privateMethod(this, _updateResult, updateResult_fn).call(this);
    __privateMethod(this, _notify3, notify_fn3).call(this);
  }
  mutate(variables, options) {
    var _a12;
    __privateSet(this, _mutateOptions, options);
    (_a12 = __privateGet(this, _currentMutation)) == null ? void 0 : _a12.removeObserver(this);
    __privateSet(this, _currentMutation, __privateGet(this, _client3).getMutationCache().build(__privateGet(this, _client3), this.options));
    __privateGet(this, _currentMutation).addObserver(this);
    return __privateGet(this, _currentMutation).execute(variables);
  }
}, _client3 = new WeakMap(), _currentResult2 = new WeakMap(), _currentMutation = new WeakMap(), _mutateOptions = new WeakMap(), _updateResult = new WeakSet(), updateResult_fn = function() {
  var _a12;
  const state = ((_a12 = __privateGet(this, _currentMutation)) == null ? void 0 : _a12.state) ?? getDefaultState2();
  __privateSet(this, _currentResult2, {
    ...state,
    isPending: state.status === "pending",
    isSuccess: state.status === "success",
    isError: state.status === "error",
    isIdle: state.status === "idle",
    mutate: this.mutate,
    reset: this.reset
  });
}, _notify3 = new WeakSet(), notify_fn3 = function(action) {
  notifyManager.batch(() => {
    var _a12, _b, _c, _d, _e, _f, _g, _h;
    if (__privateGet(this, _mutateOptions) && this.hasListeners()) {
      const variables = __privateGet(this, _currentResult2).variables;
      const context = __privateGet(this, _currentResult2).context;
      if ((action == null ? void 0 : action.type) === "success") {
        (_b = (_a12 = __privateGet(this, _mutateOptions)).onSuccess) == null ? void 0 : _b.call(_a12, action.data, variables, context);
        (_d = (_c = __privateGet(this, _mutateOptions)).onSettled) == null ? void 0 : _d.call(_c, action.data, null, variables, context);
      } else if ((action == null ? void 0 : action.type) === "error") {
        (_f = (_e = __privateGet(this, _mutateOptions)).onError) == null ? void 0 : _f.call(_e, action.error, variables, context);
        (_h = (_g = __privateGet(this, _mutateOptions)).onSettled) == null ? void 0 : _h.call(
          _g,
          void 0,
          action.error,
          variables,
          context
        );
      }
    }
    this.listeners.forEach((listener) => {
      listener(__privateGet(this, _currentResult2));
    });
  });
}, _a11);

// node_modules/.pnpm/@tanstack+query-core@5.17.8/node_modules/@tanstack/query-core/build/modern/hydration.js
function dehydrateMutation(mutation) {
  return {
    mutationKey: mutation.options.mutationKey,
    state: mutation.state,
    ...mutation.meta && { meta: mutation.meta }
  };
}
function dehydrateQuery(query) {
  return {
    state: query.state,
    queryKey: query.queryKey,
    queryHash: query.queryHash,
    ...query.meta && { meta: query.meta }
  };
}
function defaultShouldDehydrateMutation(mutation) {
  return mutation.state.isPaused;
}
function defaultShouldDehydrateQuery(query) {
  return query.state.status === "success";
}
function dehydrate(client, options = {}) {
  const filterMutation = options.shouldDehydrateMutation ?? defaultShouldDehydrateMutation;
  const mutations = client.getMutationCache().getAll().flatMap(
    (mutation) => filterMutation(mutation) ? [dehydrateMutation(mutation)] : []
  );
  const filterQuery = options.shouldDehydrateQuery ?? defaultShouldDehydrateQuery;
  const queries = client.getQueryCache().getAll().flatMap((query) => filterQuery(query) ? [dehydrateQuery(query)] : []);
  return { mutations, queries };
}
function hydrate(client, dehydratedState, options) {
  if (typeof dehydratedState !== "object" || dehydratedState === null) {
    return;
  }
  const mutationCache = client.getMutationCache();
  const queryCache = client.getQueryCache();
  const mutations = dehydratedState.mutations || [];
  const queries = dehydratedState.queries || [];
  mutations.forEach((dehydratedMutation) => {
    var _a12;
    mutationCache.build(
      client,
      {
        ...(_a12 = options == null ? void 0 : options.defaultOptions) == null ? void 0 : _a12.mutations,
        mutationKey: dehydratedMutation.mutationKey,
        meta: dehydratedMutation.meta
      },
      dehydratedMutation.state
    );
  });
  queries.forEach(({ queryKey, state, queryHash, meta }) => {
    var _a12;
    const query = queryCache.get(queryHash);
    if (query) {
      if (query.state.dataUpdatedAt < state.dataUpdatedAt) {
        const { fetchStatus: _ignored, ...dehydratedQueryState } = state;
        query.setState(dehydratedQueryState);
      }
      return;
    }
    queryCache.build(
      client,
      {
        ...(_a12 = options == null ? void 0 : options.defaultOptions) == null ? void 0 : _a12.queries,
        queryKey,
        queryHash,
        meta
      },
      // Reset fetch status to idle to avoid
      // query being stuck in fetching state upon hydration
      {
        ...state,
        fetchStatus: "idle"
      }
    );
  });
}

// node_modules/.pnpm/@tanstack+vue-query@5.17.8_vue@3.4.5/node_modules/@tanstack/vue-query/build/modern/utils.js
var VUE_QUERY_CLIENT = "VUE_QUERY_CLIENT";
function getClientKey(key) {
  const suffix = key ? `:${key}` : "";
  return `${VUE_QUERY_CLIENT}${suffix}`;
}
function updateState(state, update) {
  Object.keys(state).forEach((key) => {
    state[key] = update[key];
  });
}
function cloneDeep(value, customizer) {
  if (customizer) {
    const result = customizer(value);
    if (result === void 0 && isRef(value)) {
      return result;
    }
    if (result !== void 0) {
      return result;
    }
  }
  if (Array.isArray(value)) {
    return value.map((val) => cloneDeep(val, customizer));
  }
  if (typeof value === "object" && isPlainObject2(value)) {
    const entries = Object.entries(value).map(([key, val]) => [
      key,
      cloneDeep(val, customizer)
    ]);
    return Object.fromEntries(entries);
  }
  return value;
}
function cloneDeepUnref(obj) {
  return cloneDeep(obj, (val) => {
    if (isRef(val)) {
      return cloneDeepUnref(unref(val));
    }
    return void 0;
  });
}
function isPlainObject2(value) {
  if (Object.prototype.toString.call(value) !== "[object Object]") {
    return false;
  }
  const prototype = Object.getPrototypeOf(value);
  return prototype === null || prototype === Object.prototype;
}
function shouldThrowError(throwOnError, params) {
  if (typeof throwOnError === "function") {
    return throwOnError(...params);
  }
  return !!throwOnError;
}

// node_modules/.pnpm/@tanstack+vue-query@5.17.8_vue@3.4.5/node_modules/@tanstack/vue-query/build/modern/useQueryClient.js
function useQueryClient(id = "") {
  if (!hasInjectionContext()) {
    throw new Error(
      "vue-query hooks can only be used inside setup() function or functions that support injection context."
    );
  }
  const key = getClientKey(id);
  const queryClient = inject(key);
  if (!queryClient) {
    throw new Error(
      "No 'queryClient' found in Vue context, use 'VueQueryPlugin' to properly initialize the library."
    );
  }
  return queryClient;
}

// node_modules/.pnpm/@tanstack+vue-query@5.17.8_vue@3.4.5/node_modules/@tanstack/vue-query/build/modern/queryCache.js
var QueryCache2 = class extends QueryCache {
  find(filters) {
    return super.find(cloneDeepUnref(filters));
  }
  findAll(filters = {}) {
    return super.findAll(cloneDeepUnref(filters));
  }
};

// node_modules/.pnpm/@tanstack+vue-query@5.17.8_vue@3.4.5/node_modules/@tanstack/vue-query/build/modern/mutationCache.js
var MutationCache2 = class extends MutationCache {
  find(filters) {
    return super.find(cloneDeepUnref(filters));
  }
  findAll(filters = {}) {
    return super.findAll(cloneDeepUnref(filters));
  }
};

// node_modules/.pnpm/@tanstack+vue-query@5.17.8_vue@3.4.5/node_modules/@tanstack/vue-query/build/modern/queryClient.js
var QueryClient2 = class extends QueryClient {
  constructor(config = {}) {
    const vueQueryConfig = {
      defaultOptions: config.defaultOptions,
      queryCache: config.queryCache || new QueryCache2(),
      mutationCache: config.mutationCache || new MutationCache2()
    };
    super(vueQueryConfig);
    this.isRestoring = ref(false);
  }
  isFetching(filters = {}) {
    return super.isFetching(cloneDeepUnref(filters));
  }
  isMutating(filters = {}) {
    return super.isMutating(cloneDeepUnref(filters));
  }
  getQueryData(queryKey) {
    return super.getQueryData(cloneDeepUnref(queryKey));
  }
  ensureQueryData(options) {
    return super.ensureQueryData(cloneDeepUnref(options));
  }
  getQueriesData(filters) {
    return super.getQueriesData(cloneDeepUnref(filters));
  }
  setQueryData(queryKey, updater, options = {}) {
    return super.setQueryData(
      cloneDeepUnref(queryKey),
      updater,
      cloneDeepUnref(options)
    );
  }
  setQueriesData(filters, updater, options = {}) {
    return super.setQueriesData(
      cloneDeepUnref(filters),
      updater,
      cloneDeepUnref(options)
    );
  }
  getQueryState(queryKey) {
    return super.getQueryState(cloneDeepUnref(queryKey));
  }
  removeQueries(filters = {}) {
    return super.removeQueries(cloneDeepUnref(filters));
  }
  resetQueries(filters = {}, options = {}) {
    return super.resetQueries(cloneDeepUnref(filters), cloneDeepUnref(options));
  }
  cancelQueries(filters = {}, options = {}) {
    return super.cancelQueries(cloneDeepUnref(filters), cloneDeepUnref(options));
  }
  invalidateQueries(filters = {}, options = {}) {
    return new Promise((resolve) => {
      setTimeout(async () => {
        await super.invalidateQueries(
          cloneDeepUnref(filters),
          cloneDeepUnref(options)
        );
        resolve();
      }, 0);
    });
  }
  refetchQueries(filters = {}, options = {}) {
    return super.refetchQueries(
      cloneDeepUnref(filters),
      cloneDeepUnref(options)
    );
  }
  fetchQuery(options) {
    return super.fetchQuery(cloneDeepUnref(options));
  }
  prefetchQuery(options) {
    return super.prefetchQuery(cloneDeepUnref(options));
  }
  fetchInfiniteQuery(options) {
    return super.fetchInfiniteQuery(cloneDeepUnref(options));
  }
  prefetchInfiniteQuery(options) {
    return super.prefetchInfiniteQuery(cloneDeepUnref(options));
  }
  setDefaultOptions(options) {
    super.setDefaultOptions(cloneDeepUnref(options));
  }
  setQueryDefaults(queryKey, options) {
    super.setQueryDefaults(cloneDeepUnref(queryKey), cloneDeepUnref(options));
  }
  getQueryDefaults(queryKey) {
    return super.getQueryDefaults(cloneDeepUnref(queryKey));
  }
  setMutationDefaults(mutationKey, options) {
    super.setMutationDefaults(
      cloneDeepUnref(mutationKey),
      cloneDeepUnref(options)
    );
  }
  getMutationDefaults(mutationKey) {
    return super.getMutationDefaults(cloneDeepUnref(mutationKey));
  }
};

// node_modules/.pnpm/@tanstack+match-sorter-utils@8.8.4/node_modules/@tanstack/match-sorter-utils/build/lib/index.mjs
var characterMap = {
  À: "A",
  Á: "A",
  Â: "A",
  Ã: "A",
  Ä: "A",
  Å: "A",
  Ấ: "A",
  Ắ: "A",
  Ẳ: "A",
  Ẵ: "A",
  Ặ: "A",
  Æ: "AE",
  Ầ: "A",
  Ằ: "A",
  Ȃ: "A",
  Ç: "C",
  Ḉ: "C",
  È: "E",
  É: "E",
  Ê: "E",
  Ë: "E",
  Ế: "E",
  Ḗ: "E",
  Ề: "E",
  Ḕ: "E",
  Ḝ: "E",
  Ȇ: "E",
  Ì: "I",
  Í: "I",
  Î: "I",
  Ï: "I",
  Ḯ: "I",
  Ȋ: "I",
  Ð: "D",
  Ñ: "N",
  Ò: "O",
  Ó: "O",
  Ô: "O",
  Õ: "O",
  Ö: "O",
  Ø: "O",
  Ố: "O",
  Ṍ: "O",
  Ṓ: "O",
  Ȏ: "O",
  Ù: "U",
  Ú: "U",
  Û: "U",
  Ü: "U",
  Ý: "Y",
  à: "a",
  á: "a",
  â: "a",
  ã: "a",
  ä: "a",
  å: "a",
  ấ: "a",
  ắ: "a",
  ẳ: "a",
  ẵ: "a",
  ặ: "a",
  æ: "ae",
  ầ: "a",
  ằ: "a",
  ȃ: "a",
  ç: "c",
  ḉ: "c",
  è: "e",
  é: "e",
  ê: "e",
  ë: "e",
  ế: "e",
  ḗ: "e",
  ề: "e",
  ḕ: "e",
  ḝ: "e",
  ȇ: "e",
  ì: "i",
  í: "i",
  î: "i",
  ï: "i",
  ḯ: "i",
  ȋ: "i",
  ð: "d",
  ñ: "n",
  ò: "o",
  ó: "o",
  ô: "o",
  õ: "o",
  ö: "o",
  ø: "o",
  ố: "o",
  ṍ: "o",
  ṓ: "o",
  ȏ: "o",
  ù: "u",
  ú: "u",
  û: "u",
  ü: "u",
  ý: "y",
  ÿ: "y",
  Ā: "A",
  ā: "a",
  Ă: "A",
  ă: "a",
  Ą: "A",
  ą: "a",
  Ć: "C",
  ć: "c",
  Ĉ: "C",
  ĉ: "c",
  Ċ: "C",
  ċ: "c",
  Č: "C",
  č: "c",
  C̆: "C",
  c̆: "c",
  Ď: "D",
  ď: "d",
  Đ: "D",
  đ: "d",
  Ē: "E",
  ē: "e",
  Ĕ: "E",
  ĕ: "e",
  Ė: "E",
  ė: "e",
  Ę: "E",
  ę: "e",
  Ě: "E",
  ě: "e",
  Ĝ: "G",
  Ǵ: "G",
  ĝ: "g",
  ǵ: "g",
  Ğ: "G",
  ğ: "g",
  Ġ: "G",
  ġ: "g",
  Ģ: "G",
  ģ: "g",
  Ĥ: "H",
  ĥ: "h",
  Ħ: "H",
  ħ: "h",
  Ḫ: "H",
  ḫ: "h",
  Ĩ: "I",
  ĩ: "i",
  Ī: "I",
  ī: "i",
  Ĭ: "I",
  ĭ: "i",
  Į: "I",
  į: "i",
  İ: "I",
  ı: "i",
  Ĳ: "IJ",
  ĳ: "ij",
  Ĵ: "J",
  ĵ: "j",
  Ķ: "K",
  ķ: "k",
  Ḱ: "K",
  ḱ: "k",
  K̆: "K",
  k̆: "k",
  Ĺ: "L",
  ĺ: "l",
  Ļ: "L",
  ļ: "l",
  Ľ: "L",
  ľ: "l",
  Ŀ: "L",
  ŀ: "l",
  Ł: "l",
  ł: "l",
  Ḿ: "M",
  ḿ: "m",
  M̆: "M",
  m̆: "m",
  Ń: "N",
  ń: "n",
  Ņ: "N",
  ņ: "n",
  Ň: "N",
  ň: "n",
  ŉ: "n",
  N̆: "N",
  n̆: "n",
  Ō: "O",
  ō: "o",
  Ŏ: "O",
  ŏ: "o",
  Ő: "O",
  ő: "o",
  Œ: "OE",
  œ: "oe",
  P̆: "P",
  p̆: "p",
  Ŕ: "R",
  ŕ: "r",
  Ŗ: "R",
  ŗ: "r",
  Ř: "R",
  ř: "r",
  R̆: "R",
  r̆: "r",
  Ȓ: "R",
  ȓ: "r",
  Ś: "S",
  ś: "s",
  Ŝ: "S",
  ŝ: "s",
  Ş: "S",
  Ș: "S",
  ș: "s",
  ş: "s",
  Š: "S",
  š: "s",
  Ţ: "T",
  ţ: "t",
  ț: "t",
  Ț: "T",
  Ť: "T",
  ť: "t",
  Ŧ: "T",
  ŧ: "t",
  T̆: "T",
  t̆: "t",
  Ũ: "U",
  ũ: "u",
  Ū: "U",
  ū: "u",
  Ŭ: "U",
  ŭ: "u",
  Ů: "U",
  ů: "u",
  Ű: "U",
  ű: "u",
  Ų: "U",
  ų: "u",
  Ȗ: "U",
  ȗ: "u",
  V̆: "V",
  v̆: "v",
  Ŵ: "W",
  ŵ: "w",
  Ẃ: "W",
  ẃ: "w",
  X̆: "X",
  x̆: "x",
  Ŷ: "Y",
  ŷ: "y",
  Ÿ: "Y",
  Y̆: "Y",
  y̆: "y",
  Ź: "Z",
  ź: "z",
  Ż: "Z",
  ż: "z",
  Ž: "Z",
  ž: "z",
  ſ: "s",
  ƒ: "f",
  Ơ: "O",
  ơ: "o",
  Ư: "U",
  ư: "u",
  Ǎ: "A",
  ǎ: "a",
  Ǐ: "I",
  ǐ: "i",
  Ǒ: "O",
  ǒ: "o",
  Ǔ: "U",
  ǔ: "u",
  Ǖ: "U",
  ǖ: "u",
  Ǘ: "U",
  ǘ: "u",
  Ǚ: "U",
  ǚ: "u",
  Ǜ: "U",
  ǜ: "u",
  Ứ: "U",
  ứ: "u",
  Ṹ: "U",
  ṹ: "u",
  Ǻ: "A",
  ǻ: "a",
  Ǽ: "AE",
  ǽ: "ae",
  Ǿ: "O",
  ǿ: "o",
  Þ: "TH",
  þ: "th",
  Ṕ: "P",
  ṕ: "p",
  Ṥ: "S",
  ṥ: "s",
  X́: "X",
  x́: "x",
  Ѓ: "Г",
  ѓ: "г",
  Ќ: "К",
  ќ: "к",
  A̋: "A",
  a̋: "a",
  E̋: "E",
  e̋: "e",
  I̋: "I",
  i̋: "i",
  Ǹ: "N",
  ǹ: "n",
  Ồ: "O",
  ồ: "o",
  Ṑ: "O",
  ṑ: "o",
  Ừ: "U",
  ừ: "u",
  Ẁ: "W",
  ẁ: "w",
  Ỳ: "Y",
  ỳ: "y",
  Ȁ: "A",
  ȁ: "a",
  Ȅ: "E",
  ȅ: "e",
  Ȉ: "I",
  ȉ: "i",
  Ȍ: "O",
  ȍ: "o",
  Ȑ: "R",
  ȑ: "r",
  Ȕ: "U",
  ȕ: "u",
  B̌: "B",
  b̌: "b",
  Č̣: "C",
  č̣: "c",
  Ê̌: "E",
  ê̌: "e",
  F̌: "F",
  f̌: "f",
  Ǧ: "G",
  ǧ: "g",
  Ȟ: "H",
  ȟ: "h",
  J̌: "J",
  ǰ: "j",
  Ǩ: "K",
  ǩ: "k",
  M̌: "M",
  m̌: "m",
  P̌: "P",
  p̌: "p",
  Q̌: "Q",
  q̌: "q",
  Ř̩: "R",
  ř̩: "r",
  Ṧ: "S",
  ṧ: "s",
  V̌: "V",
  v̌: "v",
  W̌: "W",
  w̌: "w",
  X̌: "X",
  x̌: "x",
  Y̌: "Y",
  y̌: "y",
  A̧: "A",
  a̧: "a",
  B̧: "B",
  b̧: "b",
  Ḑ: "D",
  ḑ: "d",
  Ȩ: "E",
  ȩ: "e",
  Ɛ̧: "E",
  ɛ̧: "e",
  Ḩ: "H",
  ḩ: "h",
  I̧: "I",
  i̧: "i",
  Ɨ̧: "I",
  ɨ̧: "i",
  M̧: "M",
  m̧: "m",
  O̧: "O",
  o̧: "o",
  Q̧: "Q",
  q̧: "q",
  U̧: "U",
  u̧: "u",
  X̧: "X",
  x̧: "x",
  Z̧: "Z",
  z̧: "z"
};
var chars = Object.keys(characterMap).join("|");
var allAccents = new RegExp(chars, "g");
function removeAccents(str) {
  return str.replace(allAccents, (match) => {
    return characterMap[match];
  });
}
var rankings = {
  CASE_SENSITIVE_EQUAL: 7,
  EQUAL: 6,
  STARTS_WITH: 5,
  WORD_STARTS_WITH: 4,
  CONTAINS: 3,
  ACRONYM: 2,
  MATCHES: 1,
  NO_MATCH: 0
};
function rankItem(item, value, options) {
  var _options$threshold;
  options = options || {};
  options.threshold = (_options$threshold = options.threshold) != null ? _options$threshold : rankings.MATCHES;
  if (!options.accessors) {
    const rank = getMatchRanking(item, value, options);
    return {
      // ends up being duplicate of 'item' in matches but consistent
      rankedValue: item,
      rank,
      accessorIndex: -1,
      accessorThreshold: options.threshold,
      passed: rank >= options.threshold
    };
  }
  const valuesToRank = getAllValuesToRank(item, options.accessors);
  const rankingInfo = {
    rankedValue: item,
    rank: rankings.NO_MATCH,
    accessorIndex: -1,
    accessorThreshold: options.threshold,
    passed: false
  };
  for (let i = 0; i < valuesToRank.length; i++) {
    const rankValue = valuesToRank[i];
    let newRank = getMatchRanking(rankValue.itemValue, value, options);
    const {
      minRanking,
      maxRanking,
      threshold = options.threshold
    } = rankValue.attributes;
    if (newRank < minRanking && newRank >= rankings.MATCHES) {
      newRank = minRanking;
    } else if (newRank > maxRanking) {
      newRank = maxRanking;
    }
    newRank = Math.min(newRank, maxRanking);
    if (newRank >= threshold && newRank > rankingInfo.rank) {
      rankingInfo.rank = newRank;
      rankingInfo.passed = true;
      rankingInfo.accessorIndex = i;
      rankingInfo.accessorThreshold = threshold;
      rankingInfo.rankedValue = rankValue.itemValue;
    }
  }
  return rankingInfo;
}
function getMatchRanking(testString, stringToRank, options) {
  testString = prepareValueForComparison(testString, options);
  stringToRank = prepareValueForComparison(stringToRank, options);
  if (stringToRank.length > testString.length) {
    return rankings.NO_MATCH;
  }
  if (testString === stringToRank) {
    return rankings.CASE_SENSITIVE_EQUAL;
  }
  testString = testString.toLowerCase();
  stringToRank = stringToRank.toLowerCase();
  if (testString === stringToRank) {
    return rankings.EQUAL;
  }
  if (testString.startsWith(stringToRank)) {
    return rankings.STARTS_WITH;
  }
  if (testString.includes(` ${stringToRank}`)) {
    return rankings.WORD_STARTS_WITH;
  }
  if (testString.includes(stringToRank)) {
    return rankings.CONTAINS;
  } else if (stringToRank.length === 1) {
    return rankings.NO_MATCH;
  }
  if (getAcronym(testString).includes(stringToRank)) {
    return rankings.ACRONYM;
  }
  return getClosenessRanking(testString, stringToRank);
}
function getAcronym(string) {
  let acronym = "";
  const wordsInString = string.split(" ");
  wordsInString.forEach((wordInString) => {
    const splitByHyphenWords = wordInString.split("-");
    splitByHyphenWords.forEach((splitByHyphenWord) => {
      acronym += splitByHyphenWord.substr(0, 1);
    });
  });
  return acronym;
}
function getClosenessRanking(testString, stringToRank) {
  let matchingInOrderCharCount = 0;
  let charNumber = 0;
  function findMatchingCharacter(matchChar, string, index) {
    for (let j = index, J = string.length; j < J; j++) {
      const stringChar = string[j];
      if (stringChar === matchChar) {
        matchingInOrderCharCount += 1;
        return j + 1;
      }
    }
    return -1;
  }
  function getRanking(spread2) {
    const spreadPercentage = 1 / spread2;
    const inOrderPercentage = matchingInOrderCharCount / stringToRank.length;
    const ranking = rankings.MATCHES + inOrderPercentage * spreadPercentage;
    return ranking;
  }
  const firstIndex = findMatchingCharacter(stringToRank[0], testString, 0);
  if (firstIndex < 0) {
    return rankings.NO_MATCH;
  }
  charNumber = firstIndex;
  for (let i = 1, I = stringToRank.length; i < I; i++) {
    const matchChar = stringToRank[i];
    charNumber = findMatchingCharacter(matchChar, testString, charNumber);
    const found = charNumber > -1;
    if (!found) {
      return rankings.NO_MATCH;
    }
  }
  const spread = charNumber - firstIndex;
  return getRanking(spread);
}
function prepareValueForComparison(value, _ref) {
  let {
    keepDiacritics
  } = _ref;
  value = `${value}`;
  if (!keepDiacritics) {
    value = removeAccents(value);
  }
  return value;
}
function getItemValues(item, accessor) {
  let accessorFn = accessor;
  if (typeof accessor === "object") {
    accessorFn = accessor.accessor;
  }
  const value = accessorFn(item);
  if (value == null) {
    return [];
  }
  if (Array.isArray(value)) {
    return value;
  }
  return [String(value)];
}
function getAllValuesToRank(item, accessors) {
  const allValues = [];
  for (let j = 0, J = accessors.length; j < J; j++) {
    const accessor = accessors[j];
    const attributes = getAccessorAttributes(accessor);
    const itemValues = getItemValues(item, accessor);
    for (let i = 0, I = itemValues.length; i < I; i++) {
      allValues.push({
        itemValue: itemValues[i],
        attributes
      });
    }
  }
  return allValues;
}
var defaultKeyAttributes = {
  maxRanking: Infinity,
  minRanking: -Infinity
};
function getAccessorAttributes(accessor) {
  if (typeof accessor === "function") {
    return defaultKeyAttributes;
  }
  return {
    ...defaultKeyAttributes,
    ...accessor
  };
}

// node_modules/.pnpm/@tanstack+vue-query@5.17.8_vue@3.4.5/node_modules/@tanstack/vue-query/build/modern/devtools/utils.js
function getQueryState(query) {
  if (query.state.fetchStatus === "fetching") {
    return 0;
  }
  if (query.state.fetchStatus === "paused") {
    return 4;
  }
  if (!query.getObserversCount()) {
    return 3;
  }
  if (query.isStale()) {
    return 2;
  }
  return 1;
}
function getQueryStateLabel(query) {
  const queryState = getQueryState(query);
  if (queryState === 0) {
    return "fetching";
  }
  if (queryState === 4) {
    return "paused";
  }
  if (queryState === 2) {
    return "stale";
  }
  if (queryState === 3) {
    return "inactive";
  }
  return "fresh";
}
function getQueryStatusFg(query) {
  const queryState = getQueryState(query);
  if (queryState === 2) {
    return 0;
  }
  return 16777215;
}
function getQueryStatusBg(query) {
  const queryState = getQueryState(query);
  if (queryState === 0) {
    return 27647;
  }
  if (queryState === 4) {
    return 9193963;
  }
  if (queryState === 2) {
    return 16757248;
  }
  if (queryState === 3) {
    return 4148832;
  }
  return 33575;
}
var queryHashSort = (a, b) => a.queryHash.localeCompare(b.queryHash);
var dateSort = (a, b) => a.state.dataUpdatedAt < b.state.dataUpdatedAt ? 1 : -1;
var statusAndDateSort = (a, b) => {
  if (getQueryState(a) === getQueryState(b)) {
    return dateSort(a, b);
  }
  return getQueryState(a) > getQueryState(b) ? 1 : -1;
};
var sortFns = {
  "Status > Last Updated": statusAndDateSort,
  "Query Hash": queryHashSort,
  "Last Updated": dateSort
};

// node_modules/.pnpm/@tanstack+vue-query@5.17.8_vue@3.4.5/node_modules/@tanstack/vue-query/build/modern/devtools/devtools.js
var pluginId = "vue-query";
var pluginName = "Vue Query";
function setupDevtools(app, queryClient) {
  setupDevtoolsPlugin(
    {
      id: pluginId,
      label: pluginName,
      packageName: "vue-query",
      homepage: "https://tanstack.com/query/latest",
      logo: "https://raw.githubusercontent.com/TanStack/query/main/packages/vue-query/media/vue-query.svg",
      app,
      settings: {
        baseSort: {
          type: "choice",
          component: "button-group",
          label: "Sort Cache Entries",
          options: [
            {
              label: "ASC",
              value: 1
            },
            {
              label: "DESC",
              value: -1
            }
          ],
          defaultValue: 1
        },
        sortFn: {
          type: "choice",
          label: "Sort Function",
          options: Object.keys(sortFns).map((key) => ({
            label: key,
            value: key
          })),
          defaultValue: Object.keys(sortFns)[0]
        },
        onlineMode: {
          type: "choice",
          component: "button-group",
          label: "Online mode",
          options: [
            {
              label: "Online",
              value: 1
            },
            {
              label: "Offline",
              value: 0
            }
          ],
          defaultValue: 1
        }
      }
    },
    (api) => {
      const initialSettings = api.getSettings();
      onlineManager.setOnline(Boolean(initialSettings.onlineMode.valueOf()));
      const queryCache = queryClient.getQueryCache();
      api.addInspector({
        id: pluginId,
        label: pluginName,
        icon: "api",
        nodeActions: [
          {
            icon: "file_download",
            tooltip: "Refetch",
            action: (queryHash) => {
              var _a12;
              (_a12 = queryCache.get(queryHash)) == null ? void 0 : _a12.fetch();
            }
          },
          {
            icon: "alarm",
            tooltip: "Invalidate",
            action: (queryHash) => {
              const query = queryCache.get(queryHash);
              queryClient.invalidateQueries(query);
            }
          },
          {
            icon: "settings_backup_restore",
            tooltip: "Reset",
            action: (queryHash) => {
              var _a12;
              (_a12 = queryCache.get(queryHash)) == null ? void 0 : _a12.reset();
            }
          },
          {
            icon: "delete",
            tooltip: "Remove",
            action: (queryHash) => {
              const query = queryCache.get(queryHash);
              queryCache.remove(query);
            }
          },
          {
            icon: "hourglass_empty",
            tooltip: "Force loading",
            action: (queryHash) => {
              const query = queryCache.get(queryHash);
              query.setState({
                data: void 0,
                status: "pending"
              });
            }
          },
          {
            icon: "error_outline",
            tooltip: "Force error",
            action: (queryHash) => {
              const query = queryCache.get(queryHash);
              query.setState({
                data: void 0,
                status: "error",
                error: new Error("Unknown error from devtools")
              });
            }
          }
        ]
      });
      api.addTimelineLayer({
        id: pluginId,
        label: pluginName,
        color: 16767308
      });
      queryCache.subscribe((event) => {
        api.sendInspectorTree(pluginId);
        api.sendInspectorState(pluginId);
        const queryEvents = [
          "added",
          "removed",
          "updated"
        ];
        if (queryEvents.includes(event.type)) {
          api.addTimelineEvent({
            layerId: pluginId,
            event: {
              title: event.type,
              subtitle: event.query.queryHash,
              time: api.now(),
              data: {
                queryHash: event.query.queryHash,
                ...event
              }
            }
          });
        }
      });
      api.on.setPluginSettings((payload) => {
        if (payload.key === "onlineMode") {
          onlineManager.setOnline(Boolean(payload.newValue));
        }
      });
      api.on.getInspectorTree((payload) => {
        if (payload.inspectorId === pluginId) {
          const queries = queryCache.getAll();
          const settings = api.getSettings();
          const filtered = payload.filter ? queries.filter(
            (item) => rankItem(item.queryHash, payload.filter).passed
          ) : [...queries];
          const sorted = filtered.sort(
            (a, b) => sortFns[settings.sortFn](a, b) * settings.baseSort
          );
          const nodes = sorted.map((query) => {
            const stateLabel = getQueryStateLabel(query);
            return {
              id: query.queryHash,
              label: query.queryHash,
              tags: [
                {
                  label: `${stateLabel} [${query.getObserversCount()}]`,
                  textColor: getQueryStatusFg(query),
                  backgroundColor: getQueryStatusBg(query)
                }
              ]
            };
          });
          payload.rootNodes = nodes;
        }
      });
      api.on.getInspectorState((payload) => {
        if (payload.inspectorId === pluginId) {
          const query = queryCache.get(payload.nodeId);
          if (!query) {
            return;
          }
          payload.state = {
            " Query Details": [
              {
                key: "Query key",
                value: query.queryHash
              },
              {
                key: "Query status",
                value: getQueryStateLabel(query)
              },
              {
                key: "Observers",
                value: query.getObserversCount()
              },
              {
                key: "Last Updated",
                value: new Date(query.state.dataUpdatedAt).toLocaleTimeString()
              }
            ],
            "Data Explorer": [
              {
                key: "Data",
                value: query.state.data
              }
            ],
            "Query Explorer": [
              {
                key: "Query",
                value: query
              }
            ]
          };
        }
      });
    }
  );
}

// node_modules/.pnpm/@tanstack+vue-query@5.17.8_vue@3.4.5/node_modules/@tanstack/vue-query/build/modern/vueQueryPlugin.js
var VueQueryPlugin = {
  install: (app, options = {}) => {
    const clientKey = getClientKey(options.queryClientKey);
    let client;
    if ("queryClient" in options && options.queryClient) {
      client = options.queryClient;
    } else {
      const clientConfig = "queryClientConfig" in options ? options.queryClientConfig : void 0;
      client = new QueryClient2(clientConfig);
    }
    if (!isServer) {
      client.mount();
    }
    let persisterUnmount = () => {
    };
    if (options.clientPersister) {
      client.isRestoring.value = true;
      const [unmount, promise] = options.clientPersister(client);
      persisterUnmount = unmount;
      promise.then(() => {
        var _a12;
        client.isRestoring.value = false;
        (_a12 = options.clientPersisterOnSuccess) == null ? void 0 : _a12.call(options, client);
      });
    }
    const cleanup = () => {
      client.unmount();
      persisterUnmount();
    };
    if (app.onUnmount) {
      app.onUnmount(cleanup);
    } else {
      const originalUnmount = app.unmount;
      app.unmount = function vueQueryUnmount() {
        cleanup();
        originalUnmount();
      };
    }
    if (isVue2) {
      app.mixin({
        beforeCreate() {
          if (!this._provided) {
            const provideCache = {};
            Object.defineProperty(this, "_provided", {
              get: () => provideCache,
              set: (v) => Object.assign(provideCache, v)
            });
          }
          this._provided[clientKey] = client;
          if (true) {
            if (this === this.$root) {
              setupDevtools(this, client);
            }
          }
        }
      });
    } else {
      app.provide(clientKey, client);
      if (true) {
        setupDevtools(app, client);
      }
    }
  }
};

// node_modules/.pnpm/@tanstack+vue-query@5.17.8_vue@3.4.5/node_modules/@tanstack/vue-query/build/modern/queryOptions.js
function queryOptions(options) {
  return options;
}

// node_modules/.pnpm/@tanstack+vue-query@5.17.8_vue@3.4.5/node_modules/@tanstack/vue-query/build/modern/useBaseQuery.js
function useBaseQuery(Observer, options, queryClient) {
  if (true) {
    if (!getCurrentScope()) {
      console.warn(
        'vue-query composables like "useQuery()" should only be used inside a "setup()" function or a running effect scope. They might otherwise lead to memory leaks.'
      );
    }
  }
  const client = queryClient || useQueryClient();
  const defaultedOptions = computed(() => {
    const clonedOptions = cloneDeepUnref(options);
    if (typeof clonedOptions.enabled === "function") {
      clonedOptions.enabled = clonedOptions.enabled();
    }
    const defaulted = client.defaultQueryOptions(clonedOptions);
    defaulted._optimisticResults = client.isRestoring.value ? "isRestoring" : "optimistic";
    return defaulted;
  });
  const observer = new Observer(client, defaultedOptions.value);
  const state = reactive(observer.getCurrentResult());
  let unsubscribe = () => {
  };
  watch(
    client.isRestoring,
    (isRestoring) => {
      if (!isRestoring) {
        unsubscribe();
        unsubscribe = observer.subscribe((result) => {
          updateState(state, result);
        });
      }
    },
    { immediate: true }
  );
  const updater = () => {
    observer.setOptions(defaultedOptions.value);
    updateState(state, observer.getCurrentResult());
  };
  watch(defaultedOptions, updater);
  onScopeDispose(() => {
    unsubscribe();
  });
  const refetch = (...args) => {
    updater();
    return state.refetch(...args);
  };
  const suspense = () => {
    return new Promise(
      (resolve, reject) => {
        let stopWatch = () => {
        };
        const run = () => {
          if (defaultedOptions.value.enabled !== false) {
            observer.setOptions(defaultedOptions.value);
            const optimisticResult = observer.getOptimisticResult(
              defaultedOptions.value
            );
            if (optimisticResult.isStale) {
              stopWatch();
              observer.fetchOptimistic(defaultedOptions.value).then(resolve, reject);
            } else {
              stopWatch();
              resolve(optimisticResult);
            }
          }
        };
        run();
        stopWatch = watch(defaultedOptions, run);
      }
    );
  };
  watch(
    () => state.error,
    (error) => {
      if (state.isError && !state.isFetching && shouldThrowError(defaultedOptions.value.throwOnError, [
        error,
        observer.getCurrentQuery()
      ])) {
        throw error;
      }
    }
  );
  const object = toRefs(readonly(state));
  for (const key in state) {
    if (typeof state[key] === "function") {
      object[key] = state[key];
    }
  }
  object.suspense = suspense;
  object.refetch = refetch;
  return object;
}

// node_modules/.pnpm/@tanstack+vue-query@5.17.8_vue@3.4.5/node_modules/@tanstack/vue-query/build/modern/useQuery.js
function useQuery(options, queryClient) {
  return useBaseQuery(QueryObserver, options, queryClient);
}

// node_modules/.pnpm/@tanstack+vue-query@5.17.8_vue@3.4.5/node_modules/@tanstack/vue-query/build/modern/useQueries.js
function useQueries({
  queries,
  ...options
}, queryClient) {
  if (true) {
    if (!getCurrentScope()) {
      console.warn(
        'vue-query composables like "useQuery()" should only be used inside a "setup()" function or a running effect scope. They might otherwise lead to memory leaks.'
      );
    }
  }
  const client = queryClient || useQueryClient();
  const defaultedQueries = computed(
    () => cloneDeepUnref(queries).map((queryOptions2) => {
      if (typeof queryOptions2.enabled === "function") {
        queryOptions2.enabled = queryOptions2.enabled();
      }
      const defaulted = client.defaultQueryOptions(queryOptions2);
      defaulted._optimisticResults = client.isRestoring.value ? "isRestoring" : "optimistic";
      return defaulted;
    })
  );
  const observer = new QueriesObserver(
    client,
    defaultedQueries.value,
    options
  );
  const [, getCombinedResult] = observer.getOptimisticResult(
    defaultedQueries.value,
    options.combine
  );
  const state = ref(getCombinedResult());
  let unsubscribe = () => {
  };
  watch(
    client.isRestoring,
    (isRestoring) => {
      if (!isRestoring) {
        unsubscribe();
        unsubscribe = observer.subscribe(() => {
          const [, getCombinedResultRestoring] = observer.getOptimisticResult(
            defaultedQueries.value,
            options.combine
          );
          state.value = getCombinedResultRestoring();
        });
        const [, getCombinedResultPersisted] = observer.getOptimisticResult(
          defaultedQueries.value,
          options.combine
        );
        state.value = getCombinedResultPersisted();
      }
    },
    { immediate: true }
  );
  watch(
    defaultedQueries,
    () => {
      observer.setQueries(
        defaultedQueries.value,
        options
      );
      const [, getCombinedResultPersisted] = observer.getOptimisticResult(
        defaultedQueries.value,
        options.combine
      );
      state.value = getCombinedResultPersisted();
    },
    { flush: "sync" }
  );
  onScopeDispose(() => {
    unsubscribe();
  });
  return readonly(state);
}

// node_modules/.pnpm/@tanstack+vue-query@5.17.8_vue@3.4.5/node_modules/@tanstack/vue-query/build/modern/useInfiniteQuery.js
function useInfiniteQuery(options, queryClient) {
  return useBaseQuery(
    InfiniteQueryObserver,
    options,
    queryClient
  );
}

// node_modules/.pnpm/@tanstack+vue-query@5.17.8_vue@3.4.5/node_modules/@tanstack/vue-query/build/modern/useMutation.js
function useMutation(mutationOptions, queryClient) {
  if (true) {
    if (!getCurrentScope()) {
      console.warn(
        'vue-query composables like "useQuery()" should only be used inside a "setup()" function or a running effect scope. They might otherwise lead to memory leaks.'
      );
    }
  }
  const client = queryClient || useQueryClient();
  const options = computed(() => {
    return client.defaultMutationOptions(cloneDeepUnref(mutationOptions));
  });
  const observer = new MutationObserver(client, options.value);
  const state = reactive(observer.getCurrentResult());
  const unsubscribe = observer.subscribe((result) => {
    updateState(state, result);
  });
  const mutate = (variables, mutateOptions) => {
    observer.mutate(variables, mutateOptions).catch(() => {
    });
  };
  watch(options, () => {
    observer.setOptions(options.value);
  });
  onScopeDispose(() => {
    unsubscribe();
  });
  const resultRefs = toRefs(readonly(state));
  watch(
    () => state.error,
    (error) => {
      if (error && shouldThrowError(options.value.throwOnError, [error])) {
        throw error;
      }
    }
  );
  return {
    ...resultRefs,
    mutate,
    mutateAsync: state.mutate,
    reset: state.reset
  };
}

// node_modules/.pnpm/@tanstack+vue-query@5.17.8_vue@3.4.5/node_modules/@tanstack/vue-query/build/modern/useIsFetching.js
function useIsFetching(fetchingFilters = {}, queryClient) {
  if (true) {
    if (!getCurrentScope()) {
      console.warn(
        'vue-query composables like "useQuery()" should only be used inside a "setup()" function or a running effect scope. They might otherwise lead to memory leaks.'
      );
    }
  }
  const client = queryClient || useQueryClient();
  const isFetching = ref();
  const listener = () => {
    isFetching.value = client.isFetching(fetchingFilters);
  };
  const unsubscribe = client.getQueryCache().subscribe(listener);
  watchEffect(listener);
  onScopeDispose(() => {
    unsubscribe();
  });
  return isFetching;
}

// node_modules/.pnpm/@tanstack+vue-query@5.17.8_vue@3.4.5/node_modules/@tanstack/vue-query/build/modern/useMutationState.js
function useIsMutating(filters = {}, queryClient) {
  if (true) {
    if (!getCurrentScope()) {
      console.warn(
        'vue-query composables like "useQuery()" should only be used inside a "setup()" function or a running effect scope. They might otherwise lead to memory leaks.'
      );
    }
  }
  const client = queryClient || useQueryClient();
  const unreffedFilters = computed(() => ({
    ...cloneDeepUnref(filters),
    status: "pending"
  }));
  const mutationState = useMutationState({ filters: unreffedFilters }, client);
  const length = computed(() => mutationState.value.length);
  return length;
}
function getResult(mutationCache, options) {
  return mutationCache.findAll(options.filters).map(
    (mutation) => options.select ? options.select(
      mutation
    ) : mutation.state
  );
}
function useMutationState(options = {}, queryClient) {
  const filters = computed(() => cloneDeepUnref(options.filters));
  const mutationCache = (queryClient || useQueryClient()).getMutationCache();
  const state = ref(getResult(mutationCache, options));
  const unsubscribe = mutationCache.subscribe(() => {
    const result = getResult(mutationCache, options);
    state.value = result;
  });
  watch(filters, () => {
    state.value = getResult(mutationCache, options);
  });
  onScopeDispose(() => {
    unsubscribe();
  });
  return readonly(state);
}
export {
  CancelledError,
  InfiniteQueryObserver,
  MutationCache2 as MutationCache,
  MutationObserver,
  QueriesObserver,
  Query,
  QueryCache2 as QueryCache,
  QueryClient2 as QueryClient,
  QueryObserver,
  VUE_QUERY_CLIENT,
  VueQueryPlugin,
  defaultShouldDehydrateMutation,
  defaultShouldDehydrateQuery,
  dehydrate,
  focusManager,
  hashKey,
  hydrate,
  isCancelledError,
  isServer,
  keepPreviousData,
  matchQuery,
  notifyManager,
  onlineManager,
  queryOptions,
  replaceEqualDeep,
  useInfiniteQuery,
  useIsFetching,
  useIsMutating,
  useMutation,
  useMutationState,
  useQueries,
  useQuery,
  useQueryClient
};
/*! Bundled license information:

@tanstack/match-sorter-utils/build/lib/index.mjs:
  (**
   * match-sorter-utils
   *
   * Copyright (c) TanStack
   *
   * This source code is licensed under the MIT license found in the
   * LICENSE.md file in the root directory of this source tree.
   *
   * @license MIT
   *)
  (**
   * @name match-sorter
   * @license MIT license.
   * @copyright (c) 2099 Kent C. Dodds
   * @author Kent C. Dodds <me@kentcdodds.com> (https://kentcdodds.com)
   *)
*/
//# sourceMappingURL=@tanstack_vue-query.js.map
