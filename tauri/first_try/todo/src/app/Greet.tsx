'use client';

import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

const Greet = () => {
  const [greet, setGreet] = useState<string>();

  useEffect(() => {
    invoke<string>('greet', { name: 'Next.js' })
      .then((result) => setGreet(result))
      .catch((e) => console.log(e));
  }, []);

  return <div>{greet}</div>;
};

export default Greet;
