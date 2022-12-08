import {Button} from '@chakra-ui/react';
import {useSetAtom} from 'jotai/react';

import {addNavAtom, NavType} from '../atoms/nav';

export function Home(): JSX.Element {
  const addNav = useSetAtom(addNavAtom);

  return (
    <div>
      <Button
        onClick={() => {
          addNav({
            id: 'test',
            link: '/tab/test',
            type: NavType.SiteView,
            removable: true,
          });
        }}>
        Add
      </Button>
    </div>
  );
}
