import {test,expect} from '@playwright/test';
import crypto from 'node:crypto';

test('every packaged file matches the exact inventory and engine failure leaves legal/source routes usable',async({page,request})=>{
  const inventory=await(await request.get('inventory.json')).json();
  for(const file of inventory.files){
    const response=await request.get(file.path);expect(response.status(),file.path).toBe(200);
    const body=await response.body();expect(body.length,file.path).toBe(file.bytes);expect(crypto.createHash('sha256').update(body).digest('hex'),file.path).toBe(file.sha256);
    if(file.path.endsWith('.wasm'))expect(response.headers()['content-type']).toContain('application/wasm');
  }
  await page.route('**/worker.js',route=>route.abort());await page.goto('./');
  for(const label of ['Native Rust','Browser source','Software source ZIP','Compiled WASM','Build','Licenses & scope','Build record'])await expect(page.getByRole('link',{name:label,exact:true})).toBeVisible();
  await page.getByRole('link',{name:'Licenses & scope',exact:true}).click();await expect(page.locator('h1')).toContainText('TIGRIS Parliament software notices');
  for(const link of await page.locator('a').evaluateAll(nodes=>nodes.map(x=>x.getAttribute('href')).filter(x=>!x.startsWith('http'))))expect((await request.get(link)).ok(),link).toBe(true);
});

test('source scope and build provenance distinguish software from the omitted corpus',async({request})=>{
  const build=await(await request.get('build.json')).json();
  const included=await(await request.get('source-files.json')).json();
  const omitted=await(await request.get('omitted-files.json')).json();
  expect(included.some(x=>x.path==='tools/tigris-sim/src/lib.rs')).toBe(true);
  expect(included.some(x=>x.path==='tools/tigris-sim/src/main.rs')).toBe(true);
  expect(included.some(x=>x.path==='browser/parliament/tests/admission.rs')).toBe(true);
  expect(included.some(x=>x.path.startsWith('games/'))).toBe(false);expect(omitted.some(x=>x.path.startsWith('games/'))).toBe(true);
  expect(build.full_git_archive).toContain(build.commit);expect(build.wasm_memory_maximum_bytes).toBe(67108864);
  const source=await(await request.get('source.zip')).body();expect(crypto.createHash('sha256').update(source).digest('hex')).toBe(build.source_zip_sha256);
});
