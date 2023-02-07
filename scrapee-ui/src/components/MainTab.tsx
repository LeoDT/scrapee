import {HStack, StyleProps, LinkBox, LinkOverlay, Box} from '@chakra-ui/react';
import {useAtomValue, useSetAtom} from 'jotai';
import {Link as RouterLink, useNavigate} from 'react-router-dom';

import {currentNavAtom, navsAtom, NavType, removeNavAtom} from '../atoms/nav';

const ACTIVE_NAV_PROPS: StyleProps = {
  bgColor: 'white',
  color: 'gray.800',
};

export function MainTab(): JSX.Element {
  const navigate = useNavigate();
  const navs = useAtomValue(navsAtom);
  const currentNav = useAtomValue(currentNavAtom);
  const removeNav = useSetAtom(removeNavAtom);

  return (
    <HStack
      bgColor="gray.800"
      userSelect="none"
      alignItems="stretch"
      justifyContent="flex-start"
      overflow="hidden">
      {navs.map(nav => {
        const active = nav === currentNav;

        return (
          <LinkBox
            key={nav.id}
            px="2"
            py="1"
            pr={nav.removable ? '30px' : '2'}
            color="whiteAlpha.700"
            whiteSpace="nowrap"
            overflow="hidden"
            pos="relative"
            fontSize={18}
            flexShrink={nav.type === NavType.Home ? 0 : 1}
            {...(active ? ACTIVE_NAV_PROPS : {})}>
            <LinkOverlay as={RouterLink} to={nav.link ?? `/tab/${nav.id}`}>
              {nav.name}
            </LinkOverlay>
            {nav.removable ? (
              <Box
                pos="absolute"
                right={0}
                top={0}
                width={30}
                height="100%"
                bgGradient={
                  active
                    ? 'linear(to-r, whiteAlpha.600, white, white)'
                    : 'linear(to-r, transparent, gray.800, gray.800)'
                }
                color={active ? 'gray.800' : 'whiteAlpha.700'}
                textAlign="right"
                pr="1"
                onClick={() => {
                  if (active) {
                    let next = null;
                    const nextIndex = navs.indexOf(nav);

                    if (nextIndex === -1) {
                      next = navs[0];
                    } else if (nextIndex === navs.length - 1) {
                      next = navs[navs.length - 2];
                    } else {
                      next = navs[nextIndex + 1];
                    }

                    navigate(`/tab/${next.id}`);
                    setTimeout(() => {
                      removeNav(nav);
                    }, 0);
                  }
                }}>
                x
              </Box>
            ) : null}
          </LinkBox>
        );
      })}
    </HStack>
  );
}
