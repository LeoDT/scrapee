import {isEqual} from 'lodash-es';

import {atom} from 'jotai';

enum FieldType {
  Normal,
  Array,
}

// eslint-disable-next-line @typescript-eslint/no-empty-interface
interface FieldState {}

export function field<V = string>(initialValue?: V) {
  const valueAtom = atom<V | undefined>(initialValue);
  const stateAtom = atom<FieldState>({});

  const readAtom = atom(get => {
    const value = get(valueAtom);
    const state = get(stateAtom);

    return {
      value,
      dirty: value === initialValue,
      ...state,
    };
  });

  const writeAtom = atom(null, (_get, set, value: V) => {
    set(valueAtom, value);
  });

  const writeStateAtom = atom(null, (_get, set, state: FieldState) => {
    set(stateAtom, state);
  });

  return atom(() => ({
    type: FieldType.Normal,
    readAtom,
    writeAtom,
    writeStateAtom,
  }));
}

export function arrayField<VItem, TFields>(
  makeFields: (v: VItem) => TFields,
  initialValue?: VItem[],
) {
  const valueAtom = atom<VItem[]>(initialValue ?? []);
  const fieldsAtom = atom<TFields[]>(
    initialValue ? initialValue.map(v => makeFields(v)) : [],
  );
  const stateAtom = atom<FieldState>({});

  const readAtom = atom(get => {
    const value = get(valueAtom);
    const state = get(stateAtom);

    return {
      value,
      dirty: isEqual(value, initialValue),
      ...state,
    };
  });

  const readFieldsAtom = atom(get => get(fieldsAtom));

  const addAtom = atom(null, (get, set, toAdd: VItem) => {
    set(valueAtom, [...get(valueAtom), toAdd]);
    set(fieldsAtom, [...get(fieldsAtom), makeFields(toAdd)]);
  });

  const removeAtom = atom(null, (get, set, toRemove: number) => {
    const value = get(valueAtom);
    const fields = get(fieldsAtom);

    set(valueAtom, [...value.slice(0, toRemove), ...value.slice(toRemove + 1)]);
    set(fieldsAtom, [
      ...fields.slice(0, toRemove),
      ...fields.slice(toRemove + 1),
    ]);
  });

  const writeStateAtom = atom(null, (_get, set, state: FieldState) => {
    set(stateAtom, state);
  });

  return atom(() => ({
    type: FieldType.Array,
    readAtom,
    readFieldsAtom,
    addAtom,
    removeAtom,
    writeStateAtom,
  }));
}

export type NormalFieldAtoms<V> = ReturnType<typeof field<V>>;

export type ArrayFieldAtoms<VItem, TFields> = ReturnType<
  typeof arrayField<VItem, TFields>
>;
