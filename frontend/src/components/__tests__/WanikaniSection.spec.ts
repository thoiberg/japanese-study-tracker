import { describe, it, vi, expect, afterEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'

import WanikaniSectionVue, { type WanikaniResponse } from '../WanikaniSection.vue'
import LoadingIndicatorVue from '../LoadingIndicator.vue'
import UpdatedTimestampVue from '../UpdatedTimestamp.vue'

describe('WaniKani', () => {
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

    const wrapper = mount(WanikaniSectionVue)

    expect(wrapper.findComponent(LoadingIndicatorVue).exists()).toBe(true)
  })

  describe('when the request succeeds', () => {
    interface MockWanikaniResponse extends Partial<WanikaniResponse> {}

    function mockResponse(mockData?: MockWanikaniResponse) {
      const data: WanikaniResponse = {
        data_updated_at: mockData?.data_updated_at || '2023-06-24T06:00:00Z',
        active_review_count: mockData?.active_review_count || 15,
        active_lesson_count: mockData?.active_lesson_count || 66,
        daily_study_goal_met: mockData?.daily_study_goal_met || false
      }

      const mockResponse = {
        status: 200,
        ok: true,
        json: () => new Promise((resolve) => resolve(data))
      }
      fetchMock.mockResolvedValue(mockResponse)
    }

    it('displays the information', async () => {
      mockResponse()

      const wrapper = mount(WanikaniSectionVue)
      await flushPromises()

      expect(wrapper.text()).toContain('New Lessons: 66')
      expect(wrapper.text()).toContain('Current Reviews: 15')
      expect(wrapper.findComponent(UpdatedTimestampVue).text()).toEqual(
        'Data Fetched at: 24/6/23, 3:00 pm'
      )
    })

    describe('and the daily goat is not met', () => {
      it('does not show the indicator', async () => {
        mockResponse({
          daily_study_goal_met: false
        })

        const wrapper = mount(WanikaniSectionVue)
        await flushPromises()

        expect(wrapper.text()).not.toContain('🎉')
      })
    })

    describe('and the daily goat is met', () => {
      it('shows the indicator', async () => {
        mockResponse({ daily_study_goal_met: true })

        const wrapper = mount(WanikaniSectionVue)
        await flushPromises()

        expect(wrapper.text()).toContain('🎉')
      })
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

      const wrapper = mount(WanikaniSectionVue)
      await flushPromises()

      expect(wrapper.text()).toContain('uh oh')
    })
  })
})
