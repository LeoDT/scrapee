import {
  ReaderBlock,
  ReaderResponse,
  Reader as ServerReader,
  ReadersResponse,
} from '../server-types';
import {createEntitiesAtom, EntityCache} from '../utils/atomCachedEntity';
import {http} from '../utils/fetch';

interface Reader extends ServerReader {
  blocks: ReaderBlock[];
}

const cache = new EntityCache<Reader, number>();

export async function fetchReaders(): Promise<Reader[]> {
  const res = await http.get<ReadersResponse>('/readers');

  const readers: Reader[] = [];

  res.data.readers.forEach(({reader, blocks}) => {
    const r = {
      ...reader,
      blocks,
    };

    cache.set(r.id, r);

    readers.push(r);
  });

  return readers;
}

export async function fetchReader(id: number): Promise<Reader> {
  if (cache.has(id)) return cache.get(id) as Reader;

  const res = await http.get<ReaderResponse>(`/reader/${id}`);

  const reader = {
    ...res.data.reader,
    blocks: res.data.blocks,
  };

  cache.set(reader.id, reader);

  return reader;
}

export async function createReader(name: string): Promise<Reader> {
  const res = await http.post<ReaderResponse>(`/reader`, {
    name,
  });

  const reader = {
    ...res.data.reader,
    blocks: res.data.blocks,
  };

  cache.set(reader.id, reader);

  return reader;
}

export const readersAtom = createEntitiesAtom(cache);
