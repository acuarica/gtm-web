import { hhmm, pad0 } from '../src/format.js'
import assert from 'assert';

describe('format', () => {

  describe('pad0', () => {

    it('pads 0', () => {
      assert.equal(pad0(0), '00')
    })

    it('pads one digit', () => {
      assert.equal(pad0(1), '01')
    })

    it('pads two digits', () => {
      assert.equal(pad0(23), '23')
    })

    it('pads three digits', () => {
      assert.equal(pad0(456), '456')
    })

    it('pads four digits', () => {
      assert.equal(pad0(7890), '7890')
    })

  })

  describe('hhmm', () => {

    it('formats less than a minute to 0', () => {
      assert.equal(hhmm(0), '00h 00m')
      assert.equal(hhmm(59), '00h 00m')
    })

    it('formats down to the hour', () => {
      assert.equal(hhmm(5 * 60), '00h 05m')
    })

    it('formats down to the hour', () => {
      assert.equal(hhmm(4 * 60 * 60 + 5 * 60), '04h 05m')
    })

    it('formats two digits in hour', () => {
      assert.equal(hhmm(12 * 60 * 60 + 5 * 60), '12h 05m')
    })

    it('formats more than two digits in the hour', () => {
      assert.equal(hhmm(456 * 60 * 60 + 5 * 60), '456h 05m')
    })
  })

})
