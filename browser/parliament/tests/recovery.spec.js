import {test,expect} from '@playwright/test';
import fs from 'node:fs';
async function open(page){await page.goto('./');await expect(page.locator('#status')).toContainText('Rust table accepted');}
async function saved(page){const event=page.waitForEvent('download');await page.locator('#save').click();return fs.readFileSync(await(await event).path());}
const realWorker=`import init,{evaluate} from './pkg/tigris_parliament_workbench.js';await init();postMessage({type:'ready'});`;
function answer(delay='0'){return `self.onmessage=({data})=>{setTimeout(()=>postMessage({type:'result',id:data.id,report:JSON.parse(evaluate(data.request))}),JSON.parse(data.request).commands?.length||JSON.parse(data.request).save_text?${delay}:0);};`;}

test('new table during a delayed command fences the old worker result',async({page})=>{
  await page.route('**/worker.js',route=>route.fulfill({contentType:'text/javascript',body:realWorker+answer('700')}));
  await open(page);await page.locator('#command').fill('look');await page.locator('#send').click();await page.locator('#restart').click();
  await expect(page.locator('#status')).toContainText('Rust table accepted');await page.waitForTimeout(900);await expect(page.locator('#history li')).toHaveCount(0);
});

test('editing an import candidate fences its delayed successful result',async({page})=>{
  await page.route('**/worker.js',route=>route.fulfill({contentType:'text/javascript',body:realWorker+answer('700')}));
  await open(page);const before=await saved(page);const report=JSON.parse(await page.evaluate(async()=>{const{default:init,evaluate}=await import('./pkg/tigris_parliament_workbench.js');await init();return evaluate(JSON.stringify({commands:['look']}));}));
  await page.locator('#save-editor').fill(report.save_text);await page.locator('#import').click();await page.locator('#save-editor').fill('changed while running');
  await expect(page.locator('#status')).toContainText('Result discarded');expect(await saved(page)).toEqual(before);
});

test('real 10-second compute deadline retains exports and retry recovers',async({page})=>{
  await page.route('**/worker.js',route=>route.fulfill({contentType:'text/javascript',body:realWorker+`self.onmessage=({data})=>{if(JSON.parse(data.request).commands?.length)return;postMessage({type:'result',id:data.id,report:JSON.parse(evaluate(data.request))});};`}));
  await open(page);const before=await saved(page);await page.locator('#command').fill('look');await page.locator('#send').click();
  await expect(page.locator('#status')).toContainText('Computation exceeded 10 seconds',{timeout:12000});expect(await saved(page)).toEqual(before);
  await page.unroute('**/worker.js');await page.locator('#retry').click();await expect(page.locator('#status')).toContainText('Accepted table retained');await page.locator('#send').click();await expect(page.locator('#history li')).toHaveCount(1);
});

test('real 12-second startup deadline offers recovery',async({page})=>{
  await page.route('**/worker.js',route=>route.fulfill({contentType:'text/javascript',body:'/* no ready event */'}));
  await page.goto('./');await expect(page.locator('#status')).toContainText('Engine startup exceeded 12 seconds',{timeout:14000});await expect(page.locator('#send')).toBeDisabled();
  await page.unroute('**/worker.js');await page.locator('#retry').click();await expect(page.locator('#status')).toContainText('Rust table accepted');
});

test('real 5-second file-read deadline preserves accepted and candidate custody',async({page})=>{
  await page.addInitScript(()=>{const original=File.prototype.arrayBuffer;File.prototype.arrayBuffer=function(){return this.name==='slow.save'?new Promise(()=>{}):original.call(this);};});
  await open(page);await page.locator('#save-editor').fill('retain');const before=await saved(page);
  await page.locator('#save-file').setInputFiles({name:'slow.save',mimeType:'text/plain',buffer:Buffer.from('slow')});
  await expect(page.locator('#candidate-status')).toContainText('File read exceeded 5 seconds',{timeout:7000});await expect(page.locator('#save-editor')).toHaveValue('retain');expect(await saved(page)).toEqual(before);
});

test('an edited candidate wins over a stale file read',async({page})=>{
  await page.addInitScript(()=>{const original=File.prototype.arrayBuffer;File.prototype.arrayBuffer=async function(){const bytes=await original.call(this);await new Promise(r=>setTimeout(r,600));return bytes;};});
  await open(page);await page.locator('#save-file').setInputFiles({name:'slow.save',mimeType:'text/plain',buffer:Buffer.from('old file')});await page.locator('#save-editor').fill('new edit');await page.waitForTimeout(800);
  await expect(page.locator('#save-editor')).toHaveValue('new edit');await expect(page.locator('#original')).toBeDisabled();
});

test('worker exceptions retain accepted state and permit a clean retry',async({page})=>{
  await page.route('**/worker.js',route=>route.fulfill({contentType:'text/javascript',body:realWorker+`self.onmessage=({data})=>{if(JSON.parse(data.request).commands?.length)throw Error('injected worker fault');postMessage({type:'result',id:data.id,report:JSON.parse(evaluate(data.request))});};`}));
  await open(page);const before=await saved(page);await page.locator('#command').fill('look');await page.locator('#send').click();await expect(page.locator('#status')).toContainText('Worker error');expect(await saved(page)).toEqual(before);
  await page.unroute('**/worker.js');await page.locator('#retry').click();await expect(page.locator('#send')).toBeEnabled();await page.locator('#send').click();await expect(page.locator('#history li')).toHaveCount(1);
});
