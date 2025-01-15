// Luanti
// SPDX-License-Identifier: LGPL-2.1-or-later
// Copyright (C) 2013-2020 Minetest core developers team

#include "metricsbackend.h"
#include "util/thread.h"

/* Plain implementation */

class SimpleMetricCounter : public MetricCounter
{
public:
	SimpleMetricCounter() : MetricCounter(), m_counter(0.0) {}

	virtual ~SimpleMetricCounter() {}

	void increment(double number) override
	{
		MutexAutoLock lock(m_mutex);
		m_counter += number;
	}
	double get() const override
	{
		MutexAutoLock lock(m_mutex);
		return m_counter;
	}

private:
	mutable std::mutex m_mutex;
	double m_counter;
};

class SimpleMetricGauge : public MetricGauge
{
public:
	SimpleMetricGauge() : MetricGauge(), m_gauge(0.0) {}

	virtual ~SimpleMetricGauge() {}

	void increment(double number) override
	{
		MutexAutoLock lock(m_mutex);
		m_gauge += number;
	}
	void decrement(double number) override
	{
		MutexAutoLock lock(m_mutex);
		m_gauge -= number;
	}
	void set(double number) override
	{
		MutexAutoLock lock(m_mutex);
		m_gauge = number;
	}
	double get() const override
	{
		MutexAutoLock lock(m_mutex);
		return m_gauge;
	}

private:
	mutable std::mutex m_mutex;
	double m_gauge;
};

MetricCounterPtr MetricsBackend::addCounter(
		const std::string &name, const std::string &help_str, Labels labels)
{
	return std::make_shared<SimpleMetricCounter>();
}

MetricGaugePtr MetricsBackend::addGauge(
		const std::string &name, const std::string &help_str, Labels labels)
{
	return std::make_shared<SimpleMetricGauge>();
}
