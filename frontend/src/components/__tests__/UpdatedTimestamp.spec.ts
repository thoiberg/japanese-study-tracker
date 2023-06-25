import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'
import UpdatedTimestampVue from '../UpdatedTimestamp.vue'

describe('UpdatedTimestamp', () => {
  it('renders the timestamp', () => {
    const wrapper = mount(UpdatedTimestampVue, {
      props: {
        timeStamp: '2023-06-25T12:00:00Z'
      }
    })

    expect(wrapper.text()).toEqual('Data Fetched at: 25/6/23, 9:00 pm')
  })
})
