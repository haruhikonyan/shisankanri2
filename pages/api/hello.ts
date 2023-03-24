// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'
import { square, exec_screenshot } from '@/scraping/pkg'
type Data = {
  name: string
  square: number
  debug: number
}

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  const a = exec_screenshot()
  res.status(200).json({ name: 'John Doe', square: square(3), debug: a })
}
