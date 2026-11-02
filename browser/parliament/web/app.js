const $=id=>document.getElementById(id);
let worker, generation=0, job=0, pending=null, ready=false, startupTimer, newTableOnReady=false;
let accepted=null, acceptedRequest=null, allAttempts=[], original=null, fileRevision=0, candidateRevision=0;
const say=text=>{$('status').textContent=text;};
const exports=['save','replay','checkpoint','transcript','report','request','attempts'];
function controls(){
  $('send').disabled=!ready||!!pending;
  $('import').disabled=!ready||!!pending;
  for(const button of $('hints').querySelectorAll('button'))button.disabled=!ready||!!pending;
  for(const id of exports)$(id).disabled=!accepted;
}
function fail(text){
  if(pending)clearTimeout(pending.timer);
  pending=null;ready=false;generation++;worker?.terminate();clearTimeout(startupTimer);
  say(`${text} Accepted table and exports retained. Use Retry engine.`);controls();
}
function start(){
  worker?.terminate();clearTimeout(startupTimer);if(pending)clearTimeout(pending.timer);
  pending=null;ready=false;const epoch=++generation;
  say('Loading Rust engine…');controls();
  try{worker=new Worker(new URL('./worker.js',import.meta.url),{type:'module'});}catch(error){fail(String(error));return;}
  startupTimer=setTimeout(()=>{if(epoch===generation)fail('Engine startup exceeded 12 seconds.');},12000);
  worker.onerror=event=>{if(epoch===generation){event.preventDefault();fail(`Worker error: ${event.message}`);}};
  worker.onmessage=({data})=>{
    if(epoch!==generation)return;
    if(data.type==='ready'){
      clearTimeout(startupTimer);ready=true;controls();
      if(newTableOnReady||!accepted){newTableOnReady=false;submit({commands:[]},'new');}else say('Rust ready. Accepted table retained.');
      return;
    }
    if(data.type==='failure'){if(data.id===undefined||data.id===pending?.id)fail(`Engine failure: ${data.message}`);return;}
    if(data.type!=='result'||!pending||data.id!==pending.id)return;
    const operation=pending;clearTimeout(operation.timer);pending=null;
    if(operation.mode==='import'&&operation.revision!==candidateRevision){say('Candidate changed during computation. Result discarded; accepted table retained.');controls();return;}
    const report=data.report;
    if(report.kind!=='report'){say(`${report.kind}: ${report.message}. Accepted table retained.`);controls();return;}
    accepted=report;acceptedRequest=operation.request;
    if(operation.mode==='new'||operation.mode==='import')allAttempts=[];
    allAttempts.push(...report.attempts);
    render();say(`Rust table accepted · ${report.snapshot.turns} native turns · ${allAttempts.length} new-session attempts.`);controls();
  };
}
function submit(request,mode='command'){
  if(!ready||pending)return;
  if(mode==='command'&&accepted&&accepted.snapshot.turns+allAttempts.filter(x=>x.kind==='native_error').length+request.commands.length>128){say('Browser admission: session attempts exceed 128. Export your record; start a new table to continue.');return;}
  const raw=JSON.stringify(request);const id=++job;
  pending={id,request:raw,mode,revision:candidateRevision,timer:setTimeout(()=>fail('Computation exceeded 10 seconds.'),10000)};
  say('Rust is processing…');controls();worker.postMessage({id,request:raw});
}
function command(text){submit({save_text:accepted?.save_text,commands:[text]});}
function pre(text){const node=document.createElement('pre');node.textContent=text;return node;}
function render(){
  const snapshot=accepted.snapshot;
  $('room').textContent=snapshot.room;$('card').textContent=snapshot.room_card;$('response').textContent=snapshot.last_response;
  $('objectives').textContent=snapshot.panels.objectives.join('\n');$('map').textContent=snapshot.panels.map??'';
  $('inventory').textContent=snapshot.panels.inventory.map(x=>`${x.label}: ${x.detail}`).join('\n');
  $('resources').replaceChildren(...snapshot.panels.resources.map(x=>{const n=document.createElement('span');n.className='resource';n.textContent=`${x.label}: ${x.value}`;return n;}));
  $('hints').replaceChildren(...snapshot.commands.map(x=>{const n=document.createElement('button');n.type='button';n.textContent=x.command;n.title=x.description;n.onclick=()=>command(x.command);return n;}));
  $('history').replaceChildren(...snapshot.history.map(x=>{const n=document.createElement('li');n.append(pre(`${x.turn} · ${x.room} · ${x.command}\n${x.response}`));return n;}));
  $('attempt-list').replaceChildren(...allAttempts.map(x=>{const n=document.createElement('li');n.append(pre(`${x.input}\n${x.kind}: ${x.response??x.error}`));return n;}));
  const visuals=[];function visit(nodes){for(const x of nodes){const n=document.createElement('span');n.textContent=[x.label,x.text].filter(Boolean).join(': ');visuals.push(n);visit(x.children??[]);}}visit(snapshot.visual_nodes);$('visual').replaceChildren(...visuals);
  $('snapshot').textContent=JSON.stringify(snapshot,null,2);
}
function download(name,body,type='text/plain;charset=utf-8'){
  const blob=body instanceof Blob?body:new Blob([body],{type});const url=URL.createObjectURL(blob);const a=document.createElement('a');a.href=url;a.download=name;a.click();setTimeout(()=>URL.revokeObjectURL(url),30000);
}
const downloadFields={save:['parliament.save','save_text'],replay:['parliament-replay.save','replay_save_text'],checkpoint:['parliament-checkpoint.txt','checkpoint_text'],transcript:['parliament-transcript.txt','transcript_text']};
for(const [id,[name,field]]of Object.entries(downloadFields))$(id).onclick=()=>{if(accepted)download(name,accepted[field]??'');};
$('report').onclick=()=>{if(accepted)download('parliament-report.json',JSON.stringify(accepted,null,2),'application/json');};
$('request').onclick=()=>{if(acceptedRequest)download('parliament-request.json',acceptedRequest,'application/json');};
$('attempts').onclick=()=>{if(accepted)download('parliament-attempts.json',JSON.stringify(allAttempts,null,2),'application/json');};
$('command-form').onsubmit=event=>{event.preventDefault();command($('command').value);};
$('retry').onclick=start;
$('restart').onclick=()=>{fileRevision++;candidateRevision++;if(pending||!ready){newTableOnReady=true;start();}else submit({commands:[]},'new');};
$('save-editor').oninput=()=>{candidateRevision++;fileRevision++;$('candidate-status').textContent='Candidate edited. Original bytes remain available; accepted table retained.';};
$('import').onclick=()=>submit({save_text:$('save-editor').value,commands:[]},'import');
$('original').onclick=()=>{if(original)download(original.name,original.bytes);};
$('current').onclick=()=>download('candidate-editor-current.txt',$('save-editor').value);
$('lf').onclick=()=>download('candidate-editor-lf.txt',$('save-editor').value.replace(/\r\n?/g,'\n'));
$('save-file').onchange=async()=>{
  const file=$('save-file').files[0];if(!file)return;
  const revision=++fileRevision;candidateRevision++;
  const previous=$('candidate-status').textContent;
  $('candidate-status').textContent='Reading candidate…';
  let timer;
  try{
    if(file.size>131072)throw Error('Save exceeds 128 KiB');
    const bytes=await Promise.race([file.arrayBuffer(),new Promise((_,reject)=>{timer=setTimeout(()=>reject(Error('File read exceeded 5 seconds')),5000);})]);
    if(revision!==fileRevision)return;
    const text=new TextDecoder('utf-8',{fatal:true,ignoreBOM:true}).decode(bytes);
    original={name:file.name,bytes:new Blob([bytes])};$('original').disabled=false;
    $('save-editor').value=text;candidateRevision++;$('candidate-status').textContent=`Opened ${file.name}. Editor normalizes line endings; original UTF-8 bytes (including BOM) remain separate. Resume explicitly to change the table.`;
  }catch(error){if(revision===fileRevision)$('candidate-status').textContent=`${error.message}. Previous candidate and accepted table retained. ${previous}`;}
  finally{clearTimeout(timer);}
};
controls();start();
