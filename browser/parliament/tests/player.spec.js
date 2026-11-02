import {test,expect} from '@playwright/test';
import fs from 'node:fs';
const cases=JSON.parse(fs.readFileSync(new URL('../.cache/native-cases.json',import.meta.url),'utf8'));
async function open(page){await page.goto('./');await expect(page.locator('#status')).toContainText('Rust table accepted');}
async function send(page,text){await page.locator('#command').fill(text);await page.locator('#send').click();await expect(page.locator('#send')).toBeEnabled();}
async function download(page,id){const event=page.waitForEvent('download');await page.locator(`#${id}`).click();const file=await event;return fs.readFileSync(await file.path());}

test('real WASM matches every native response and exported memory has a 64 MiB maximum',async({page})=>{
  await open(page);
  const result=await page.evaluate(async records=>{
    const {default:init,evaluate}=await import('./pkg/tigris_parliament_workbench.js');
    const module=await init();
    const mismatches=[];
    for(let i=0;i<records.length;i++){
      const actual=JSON.parse(evaluate(records[i].request));
      if(JSON.stringify(actual)!==JSON.stringify(records[i].response))mismatches.push(i);
    }
    let refused=false;try{module.memory.grow(1024);}catch{refused=true;}
    return {mismatches,refused};
  },cases);
  expect(result).toEqual({mismatches:[],refused:true});
});

test('keyboard play and native hints show the actual table and full turn history',async({page})=>{
  await open(page);await page.getByRole('button',{name:'choose persona',exact:true}).click();
  await expect(page.locator('#history li')).toHaveCount(1);
  await page.locator('#command').fill('go board');await page.locator('#command').press('Enter');
  await expect(page.locator('#room')).toHaveText('board');
  for(const command of ['draft axis','stake claim','reveal collision','place tiger','end turn'])await send(page,command);
  await expect(page.locator('#history li')).toHaveCount(7);
  await expect(page.locator('#snapshot')).toContainText('visual_nodes');
  await expect(page.locator('#resources')).not.toBeEmpty();
});

test('native errors remain visible without fabricated turns or HTML execution',async({page})=>{
  await open(page);await send(page,'<img src=x onerror="window.injected=1">');await send(page,'look');
  await expect(page.locator('#attempt-list li')).toHaveCount(2);await expect(page.locator('#history li')).toHaveCount(1);
  await expect(page.locator('#attempt-list')).toContainText('native_error');expect(await page.evaluate(()=>window.injected)).toBeUndefined();
});

test('native save resume and all accepted exports have exact custody',async({page})=>{
  await open(page);await send(page,'choose persona');await send(page,'go board');
  const report=JSON.parse((await download(page,'report')).toString());
  for(const [id,field]of Object.entries({save:'save_text',replay:'replay_save_text',checkpoint:'checkpoint_text',transcript:'transcript_text'}))expect((await download(page,id)).toString()).toBe(report[field]);
  await page.locator('#restart').click();await expect(page.locator('#history li')).toHaveCount(0);
  await page.locator('#save-editor').fill(report.save_text);await page.locator('#import').click();await expect(page.locator('#history li')).toHaveCount(2);
  expect(JSON.parse((await download(page,'report')).toString()).save_text).toBe(report.save_text);
});

test('bad and huge-round candidates preserve accepted table and exports',async({page})=>{
  await open(page);await send(page,'look');const before=await download(page,'save');
  for(const candidate of ['broken',cases.find(x=>x.response.kind==='browser_refusal'&&x.request.includes('4294967295')).request&&JSON.parse(cases.find(x=>x.response.kind==='browser_refusal'&&x.request.includes('4294967295')).request).save_text]){
    await page.locator('#save-editor').fill(candidate);await page.locator('#import').click();await expect(page.locator('#status')).toContainText('Accepted table retained');expect(await download(page,'save')).toEqual(before);
  }
});

test('opened UTF-8 BOM and CRLF bytes are distinct from current and LF editor text',async({page})=>{
  await open(page);const bytes=Buffer.from('\ufeffMUDDLE_SESSION_V1\r\ncurrent_room=table\r\n');
  await page.locator('#save-file').setInputFiles({name:'original.save',mimeType:'text/plain',buffer:bytes});
  await expect(page.locator('#candidate-status')).toContainText('Opened original.save');expect(await download(page,'original')).toEqual(bytes);
  const current=await page.locator('#save-editor').inputValue();expect((await download(page,'current')).toString()).toBe(current);expect((await download(page,'lf')).toString()).toBe(current.replace(/\r\n?/g,'\n'));
  await page.locator('#save-editor').fill('edited');expect(await download(page,'original')).toEqual(bytes);
});

test('invalid UTF-8 and oversized files keep previous candidate and accepted save',async({page})=>{
  await open(page);await page.locator('#save-editor').fill('retain candidate');const before=await download(page,'save');
  for(const bytes of [Buffer.from([0xff]),Buffer.alloc(131073,120)]){
    await page.locator('#save-file').setInputFiles({name:'invalid.save',mimeType:'text/plain',buffer:bytes});await expect(page.locator('#candidate-status')).toContainText('Previous candidate');await expect(page.locator('#save-editor')).toHaveValue('retain candidate');expect(await download(page,'save')).toEqual(before);
  }
});

test('retry retains accepted session and new table resets it',async({page})=>{
  await open(page);await send(page,'look');const before=await download(page,'save');await page.locator('#retry').click();await expect(page.locator('#status')).toContainText('Accepted table retained');expect(await download(page,'save')).toEqual(before);
  await page.locator('#restart').click();await expect(page.locator('#history li')).toHaveCount(0);await expect(page.locator('#attempt-list li')).toHaveCount(0);
});

test('mobile task fits the viewport and keeps exports readable',async({page})=>{
  await page.setViewportSize({width:390,height:844});await open(page);await send(page,'look');
  expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
  for(const id of ['save','transcript','original','current']){const box=await page.locator(`#${id}`).boundingBox();expect(box.width).toBeGreaterThan(250);}
});
