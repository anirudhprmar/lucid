import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { load } from '@tauri-apps/plugin-store';
import { ActivityCalendar } from 'react-activity-calendar';

import type { Activity } from 'react-activity-calendar';

export default function Dashboard() {
  const [currentModel, setCurrentModel] = useState<string>('Loading...');
  const [totalTranscriptions, setTotalTranscriptions] = useState<
    number | string
  >('—');
  const [hoursTranscribed, setHoursTranscribed] = useState<number | string>(
    '—'
  );
  const [calendarData, setCalendarData] = useState<Activity[]>([]);

  useEffect(() => {
    async function getModel() {
      const model = await invoke<string | null>('get_current_model');
      if (!model) {
        setCurrentModel('No Model Found');
      } else {
        setCurrentModel(model);
      }
    }

    void getModel();
  }, []);

  useEffect(() => {
    async function loadUsageData() {
      const store = await load('usage.json', { autoSave: false });
      const entries = await store.entries<{
        count: number;
        duration: number;
      }>();

      const activityData = entries.map(([date, data]) => ({
        date: new Date(date).toISOString().slice(0, 10),
        count: data.count,
        level: Math.min(4, Math.ceil(data.count / 2)),
      }));

      activityData.sort((a, b) => a.date.localeCompare(b.date));

      const today = new Date().toISOString().slice(0, 10);
      if (activityData[activityData.length - 1].date !== today) {
        activityData.push({ date: today, count: 0, level: 0 });
      }

      setCalendarData(activityData);

      let totalCount = 0;
      let totalHours = 0;
      entries.forEach(([_, data]) => {
        totalCount += data.count;
        totalHours += data.duration;
      });
      totalHours /= 3600;
      setTotalTranscriptions(totalCount);
      setHoursTranscribed(totalHours.toFixed(1));
    }
    void loadUsageData();
  }, []);

  return (
    <div className='flex flex-1 flex-col gap-6 p-6'>
      <div className='grid grid-cols-1 gap-4 sm:grid-cols-3'>
        <StatCard
          label='Total Transcriptions'
          value={totalTranscriptions.toString()}
        />
        <StatCard
          label='Hours Transcribed'
          value={hoursTranscribed.toString()}
        />
        <StatCard
          label='Current Model'
          value={currentModel.split(/[\\/]/).pop() as string}
        />
      </div>

      <div className='w-fit rounded-2xl border border-white/10 bg-white/2 p-5'>
        <h2 className='text-sm font-medium text-white/80'>Usage Heatmap</h2>
        <p className='mt-1 text-xs text-zinc-500'>
          Your transcription activity over {calendarData.length} days.
        </p>
        <div className='mt-4 overflow-x-auto'>
          {calendarData.length > 0 ? (
            <div className='text-sm text-zinc-600'>
              <ActivityCalendar
                data={calendarData}
                colorScheme='dark'
                theme={{ dark: ['#1a1a2e', '#7c3aed'] }}
                blockSize={12}
                blockMargin={4}
                showWeekdayLabels
              />
            </div>
          ) : (
            <div className='flex h-32 items-center justify-center text-sm text-zinc-600'>
              Start using Lucid to see your activity
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

function StatCard({ label, value }: { label: string; value: string }) {
  return (
    <div className='rounded-xl border border-white/10 bg-white/2 p-4'>
      <p className='text-xs font-medium text-zinc-400'>{label}</p>
      <p className='mt-2 text-2xl font-semibold text-white'>{value}</p>
    </div>
  );
}
