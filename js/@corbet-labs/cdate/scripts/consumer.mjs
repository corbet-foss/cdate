// Copy next to a fresh installation to test the actual npm tarball.
import { createRequire } from 'node:module';
import { verify } from './verify-api.mjs';
verify(await import('@corbet-labs/cdate'));
verify(createRequire(import.meta.url)('@corbet-labs/cdate'));
verify(await import('@corbet-labs/cdate/browser'));
console.log('cdate: installed ESM, CommonJS, and browser exports passed');
