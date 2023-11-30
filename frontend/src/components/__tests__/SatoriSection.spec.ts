import { flushPromises, mount } from '@vue/test-utils'
import { afterEach, describe, vi, it, expect } from 'vitest'
import SatoriSectionVue, { type SatoriResponse } from '../SatoriSection.vue'
import LoadingIndicatorVue from '../LoadingIndicator.vue'
import UpdatedTimestampVue from '../UpdatedTimestamp.vue'
import DailyGoalIndicatorVue from '../DailyGoalIndicator.vue'

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
    interface MockSatoriResponse extends Partial<SatoriResponse> {}

    function mockResponse(mockData?: MockSatoriResponse) {
      const data: SatoriResponse = {
        data_updated_at: mockData?.data_updated_at || '2023-06-24T06:00:00Z',
        new_card_count: mockData?.new_card_count || 20,
        active_review_count: mockData?.active_review_count || 8,
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

      const wrapper = mount(SatoriSectionVue)
      await flushPromises()

      expect(wrapper.text()).toContain('Current Reviews: 8')
      expect(wrapper.text()).toContain('New Cards: 20')
      expect(wrapper.findComponent(UpdatedTimestampVue).text()).toEqual(
        'Data Fetched at: 24/6/23, 3:00 pm'
      )
    })

    describe('when the study goal has not been met', () => {
      it('does not show the study indicator', async () => {
        mockResponse({ daily_study_goal_met: false })

        const wrapper = mount(SatoriSectionVue)
        await flushPromises()

        expect(wrapper.findComponent(DailyGoalIndicatorVue).exists()).toBe(false)
      })
    })

    describe('when the study goal has met', () => {
      it('shows the study indicator', async () => {
        mockResponse({ daily_study_goal_met: true })

        const wrapper = mount(SatoriSectionVue)
        await flushPromises()

        expect(wrapper.findComponent(DailyGoalIndicatorVue).exists()).toBe(true)
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

      const wrapper = mount(SatoriSectionVue)
      await flushPromises()

      expect(wrapper.text()).toContain('uh oh')
    })
  })
})
