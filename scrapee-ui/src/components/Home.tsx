import {useCallback, useEffect, useState} from 'react';

import {
  Button,
  Box,
  Card,
  CardBody,
  Heading,
  Wrap,
  WrapItem,
  Input,
} from '@chakra-ui/react';
import {useSetAtom, useAtomValue} from 'jotai';

import {addNavAtom, NavType, ReaderNav, SiteNav} from '../atoms/nav';
import {createReader, fetchReaders, readersAtom} from '../atoms/reader';
import {fetchSites, sitesAtom} from '../atoms/site';

export function Home(): JSX.Element {
  const sites = useAtomValue(sitesAtom);
  const readers = useAtomValue(readersAtom);
  const loadSitesAndReaders = useCallback(() => {
    fetchSites();
    fetchReaders();
  }, []);
  const addNav = useSetAtom(addNavAtom);
  const addTestReaderAndNav = useCallback(async () => {
    const id = Date.now().toString();
    const reader = await createReader(new Date().toISOString());

    if (reader.id) {
      const nav: ReaderNav = {
        id,
        name: reader.name,
        link: `/tab/${id}`,
        readerId: reader.id,
        type: NavType.Reader,
        removable: true,
      };

      addNav(nav);
    }
  }, [addNav, createReader]);

  useEffect(() => {
    loadSitesAndReaders();
  }, []);

  return (
    <Box userSelect="none">
      <Heading as="h3">Sites</Heading>

      <Wrap p="4">
        {sites.map(site => (
          <WrapItem key={site.id}>
            <Card
              onClick={() => {
                const id = Date.now().toString();

                const nav: SiteNav = {
                  id,
                  name: site.name,
                  link: `/tab/${id}`,
                  siteId: site.id,
                  type: NavType.Site,
                  removable: true,
                };

                addNav(nav);
              }}
              cursor="pointer"
              _hover={{shadow: 'lg'}}
              transitionDuration="0.15s"
              transitionProperty="box-shadow"
              transitionTimingFunction="ease-in-out">
              <CardBody>{site.name}</CardBody>
            </Card>
          </WrapItem>
        ))}
      </Wrap>

      <Heading as="h3">Readers</Heading>

      <Wrap p="4">
        {readers.map(reader => (
          <WrapItem key={reader.id}>
            <Card
              onClick={() => {
                const id = Date.now().toString();

                const nav: ReaderNav = {
                  id,
                  name: reader.name,
                  link: `/tab/${id}`,
                  readerId: reader.id,
                  type: NavType.Reader,
                  removable: true,
                };

                addNav(nav);
              }}
              cursor="pointer"
              _hover={{shadow: 'lg'}}
              transitionDuration="0.15s"
              transitionProperty="box-shadow"
              transitionTimingFunction="ease-in-out">
              <CardBody>{reader.name}</CardBody>
            </Card>
          </WrapItem>
        ))}
      </Wrap>

      <Button
        onClick={() => {
          addTestReaderAndNav();
        }}>
        Add
      </Button>
    </Box>
  );
}
