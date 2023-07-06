import { flushPromises, mount } from '@vue/test-utils'
import { afterEach, describe, vi, it, expect } from 'vitest'
import SatoriSectionVue from '../SatoriSection.vue'
import LoadingIndicatorVue from '../LoadingIndicator.vue'
import UpdatedTimestampVue from '../UpdatedTimestamp.vue'

describe('SatoriSection', () => {
  const fetchMock = vi.fn()
  global.fetch = fetchMock

  afterEach(() => {
    vi.restoreAllMocks()
  })

  it('displays a loading indicator', async () => {
    const mockResponse = {
      json: () => new Promise((resolve) => resolve({}))
    }
    fetchMock.mockResolvedValue(mockResponse)

    const wrapper = mount(SatoriSectionVue)

    expect(wrapper.findComponent(LoadingIndicatorVue).exists()).toBe(true)
  })

  describe('when the request succeeds', () => {
    it('displays the information', async () => {
      const data = {
        data_updated_at: '2023-06-24T06:00:00Z',
        new_card_count: 20,
        active_review_count: 8
      }
      const mockResponse = {
        status: 200,
        ok: true,
        json: () => new Promise((resolve) => resolve(data))
      }
      fetchMock.mockResolvedValue(mockResponse)

      const wrapper = mount(SatoriSectionVue)
      await flushPromises()

      expect(wrapper.text()).toContain('Current Reviews: 8')
      expect(wrapper.text()).toContain('New Cards: 20')
      expect(wrapper.findComponent(UpdatedTimestampVue).text()).toEqual(
        'Data Fetched at: 24/6/23, 3:00 pm'
      )
    })
  })

  describe('when the request fails', () => {
    it('displays an error message', async () => {
      const data = {
        message: 'uh oh'
      }
      const mockResponse = {
        status: 500,
        ok: false,
        json: () => new Promise((resolve) => resolve(data))
      }
      fetchMock.mockResolvedValue(mockResponse)

      const wrapper = mount(SatoriSectionVue)
      await flushPromises()

      expect(wrapper.text()).toContain('uh oh')
    })
  })
})
