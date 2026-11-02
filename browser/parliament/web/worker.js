import init, {evaluate} from './pkg/tigris_parliament_workbench.js';
try {
  await init();
  self.postMessage({type:'ready'});
  self.onmessage = ({data}) => {
    try { self.postMessage({type:'result',id:data.id,report:JSON.parse(evaluate(data.request))}); }
    catch(error) { self.postMessage({type:'failure',id:data.id,message:String(error)}); }
  };
} catch(error) { self.postMessage({type:'failure',message:String(error)}); }
