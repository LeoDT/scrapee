import axios from 'axios';

export const http = axios.create({
  baseURL: `http://localhost:${__SCRAPEE_CONFIG__.port}`,
  headers: {'X-Token': __SCRAPEE_CONFIG__.token},
});
