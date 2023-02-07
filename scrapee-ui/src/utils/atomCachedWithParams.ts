import {identity} from 'lodash-es';

import {atom} from 'jotai';
import type {Atom} from 'jotai';

type Fetch<V, P> = (p: P) => Promise<V>;
interface AtomCahcedWithParamsApi<P> {
  invalidate: (p: P) => void;
}

export function atomCachedWithParams<Value, Params, ParamKey = Params>(
  fetch: Fetch<Value, Params>,
  paramsAtom: Atom<Params>,
  paramKeyExtractor: (p: Params) => ParamKey = identity,
): [Atom<Promise<Value>>, AtomCahcedWithParamsApi<Params>] {
  const cache = new Map<ParamKey, Value>();

  const baseAtom = atom(async get => {
    const params = get(paramsAtom);
    const paramKey = paramKeyExtractor(params);

    if (paramKey === null) {
      throw new Error('atom can not be cached with key = null');
    }

    if (cache.has(paramKey)) {
      return cache.get(paramKey) as Value;
    }

    const value = await fetch(params);

    cache.set(paramKey, value);

    return value;
  });

  const invalidate = (params: Params) => {
    const paramKey = paramKeyExtractor(params);
    cache.delete(paramKey);
  };

  return [baseAtom, {invalidate}];
}
