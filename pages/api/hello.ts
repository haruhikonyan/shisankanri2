// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'
// import { square, exec_screenshot } from '@/wasm/scraping/pkg/scraping'
import { square } from '@/wasm/scraping/pkg/scraping'
type Data = {
  name: string
  square: number
  // debug: number
}

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  // const a = exec_screenshot()
  // res.status(200).json({ name: 'John Doe', square: square(3), debug: a })
  res.status(200).json({ name: 'John Doe', square: square(3) })
}
