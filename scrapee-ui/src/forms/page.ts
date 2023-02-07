import {useCallback, useMemo} from 'react';

import {atom} from 'jotai';

import {Page} from '../atoms/site';
import {arrayField, field} from '../utils/form';

export function usePageFormAtom(initialValue: Partial<Page>, deps: unknown[]) {
  const createForm = useCallback((value: Partial<Page>) => {
    return {
      name: field(value.name),
      url: field(value.url),
      url_pattern: field(value.url_pattern),

      fields: arrayField(
        v => ({
          name: field(v.name),
          xpath: field(v.xpath),
          try_follow: field(v.try_follow),
        }),
        value.fields,
      ),
    };
  }, []);
  const formAtom = useMemo(() => {
    const baseAtom = atom(createForm(initialValue));

    const valueAtom = atom(get => {
      const form = get(baseAtom);

      return {
        name: get(get(form.name).readAtom).value,
        url: get(get(form.url).readAtom).value,
        url_pattern: get(get(form.url_pattern).readAtom).value,
        fields: get(get(form.fields).readAtom).value,
      };
    });

    return [baseAtom, valueAtom] as const;
  }, deps);

  return formAtom;
}
