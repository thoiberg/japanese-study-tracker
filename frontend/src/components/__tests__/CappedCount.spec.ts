import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'
import CappedCountVue from '../CappedCount.vue'

describe('CappedCount', () => {
  describe('when the total amount is greater than the capped amount', () => {
    it('displays the extra amount', () => {
      const wrapper = mount(CappedCountVue, {
        props: {
          cardType: 'New Card',
          cappedCount: 40,
          totalCount: 140
        }
      })

      expect(wrapper.text()).toEqual('New Card: 40  (+ 100)')
    })
  })

  describe('when the total amount is the same as the capped amount', () => {
    it('does not display any extra data', () => {
      const wrapper = mount(CappedCountVue, {
        props: {
          cardType: 'New Card',
          cappedCount: 40,
          totalCount: 40
        }
      })

      expect(wrapper.text()).toEqual('New Card: 40')
    })
  })
})
