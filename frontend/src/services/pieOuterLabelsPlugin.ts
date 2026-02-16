import type { Chart as ChartJS, Plugin } from 'chart.js'

export const pieOuterLabelsPlugin: Plugin<'pie'> = {
  id: 'pieOuterLabels',
  afterDraw(chart: ChartJS<'pie'>) {
    const { ctx } = chart
    const meta = chart.getDatasetMeta(0)
    if (!meta?.data?.length) return

    const total = meta.data.reduce((sum, arc) => {
      const { startAngle, endAngle } = arc.getProps(['startAngle', 'endAngle'])
      return sum + (endAngle - startAngle)
    }, 0)

    const labels = chart.data.labels as string[]
    const style = getComputedStyle(chart.canvas)
    const textColor = style.getPropertyValue('color') || '#666'

    ctx.save()
    ctx.font = '11px sans-serif'
    ctx.fillStyle = textColor
    ctx.strokeStyle = textColor
    ctx.lineWidth = 1

    for (let i = 0; i < meta.data.length; i++) {
      const arc = meta.data[i]
      const { x, y, startAngle, endAngle, outerRadius } = arc.getProps(['x', 'y', 'startAngle', 'endAngle', 'outerRadius'])

      const sliceAngle = endAngle - startAngle
      if (sliceAngle / total < 0.03) continue

      const midAngle = (startAngle + endAngle) / 2
      const cos = Math.cos(midAngle)
      const sin = Math.sin(midAngle)

      const startX = x + cos * outerRadius
      const startY = y + sin * outerRadius
      const elbowX = x + cos * (outerRadius + 15)
      const elbowY = y + sin * (outerRadius + 15)
      const endX = elbowX + (cos >= 0 ? 20 : -20)

      ctx.beginPath()
      ctx.moveTo(startX, startY)
      ctx.lineTo(elbowX, elbowY)
      ctx.lineTo(endX, elbowY)
      ctx.stroke()

      ctx.textAlign = cos >= 0 ? 'left' : 'right'
      ctx.textBaseline = 'middle'
      ctx.fillText(labels[i] ?? '', endX + (cos >= 0 ? 4 : -4), elbowY)
    }

    ctx.restore()
  },
}
