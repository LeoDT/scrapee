interface Props {
  readerId: number;
}

export function TabReader({readerId}: Props): JSX.Element {
  return <div>{readerId}</div>;
}
