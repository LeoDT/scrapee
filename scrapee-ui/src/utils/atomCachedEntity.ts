import {Atom, atom} from 'jotai';
import {atomWithProxy} from 'jotai-valtio';
import {proxyMap} from 'valtio/utils';

export class EntityCache<Entity, Key = number> {
  #cache: Map<Key, Entity>;
  rootAtom: Atom<Map<Key, Entity>>;

  constructor() {
    this.#cache = proxyMap<Key, Entity>([]);
    this.rootAtom = atomWithProxy(this.#cache);
  }

  get(id: Key) {
    return this.#cache.get(id);
  }

  set(id: Key, e: Entity) {
    this.#cache.set(id, e);
  }

  has(id: Key) {
    return this.#cache.has(id);
  }

  invalidate(id: Key) {
    this.#cache.delete(id);
  }
}

export function createEntitiesAtom<E, K>(cache: EntityCache<E, K>) {
  return atom(get => Array.from(get(cache.rootAtom).values()));
}
