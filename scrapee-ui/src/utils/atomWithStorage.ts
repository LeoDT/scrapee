import {
  createJSONStorage,
  atomWithStorage as jotaiAtomWithStorage,
  unstable_NO_STORAGE_VALUE,
} from 'jotai/utils';
import type {SyncStorage} from 'jotai/vanilla/utils/atomWithStorage';

const storage = createJSONStorage(() => window.localStorage);

export function atomWithStorage<V>(key: string, defaultValue: V) {
  let initialValue = storage.getItem(key) as V;

  if (initialValue === unstable_NO_STORAGE_VALUE) {
    initialValue = defaultValue;
    storage.setItem(key, defaultValue);
  }

  return jotaiAtomWithStorage<V>(key, initialValue, storage as SyncStorage<V>);
}
