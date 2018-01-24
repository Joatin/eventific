

function retry(callback: (retry: any) => Promise<any>) {
  return new Promise((resolve, reject)=>{
    const retryCallback = (err: any) => {
      reject(err);
    };
    callback(retryCallback).then(resolve, reject);
  });
}

export = retry;
