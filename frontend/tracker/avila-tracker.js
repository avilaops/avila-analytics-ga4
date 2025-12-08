/**
 * Avila Analytics Tracker
 * Ultra-fast, privacy-first analytics tracker
 */

(function() {
    'use strict';

    const config = {
        endpoint: window.AVILA_ENDPOINT || 'http://localhost:8080',
        measurementId: document.currentScript?.getAttribute('data-site'),
        debug: document.currentScript?.getAttribute('data-debug') === 'true',
        autoTrack: document.currentScript?.getAttribute('data-auto-track') !== 'false',
    };

    // Generate client ID
    const getClientId = () => {
        let clientId = localStorage.getItem('avila_client_id');
        if (!clientId) {
            clientId = 'cid_' + Math.random().toString(36).substr(2, 9) + Date.now();
            localStorage.setItem('avila_client_id', clientId);
        }
        return clientId;
    };

    // Generate session ID
    const getSessionId = () => {
        const SESSION_TIMEOUT = 30 * 60 * 1000; // 30 minutes
        let session = JSON.parse(sessionStorage.getItem('avila_session') || '{}');
        const now = Date.now();

        if (!session.id || (now - session.lastActivity) > SESSION_TIMEOUT) {
            session = {
                id: 'ses_' + Math.random().toString(36).substr(2, 9) + now,
                started: now,
                lastActivity: now,
            };
        } else {
            session.lastActivity = now;
        }

        sessionStorage.setItem('avila_session', JSON.stringify(session));
        return session.id;
    };

    // Get device info
    const getDeviceInfo = () => {
        const ua = navigator.userAgent;
        return {
            user_agent: ua,
            language: navigator.language,
            screen_resolution: `${screen.width}x${screen.height}`,
            viewport_size: `${window.innerWidth}x${window.innerHeight}`,
            device_category: /Mobile|Android|iPhone/i.test(ua) ? 'mobile' : 'desktop',
        };
    };

    // Track event
    const trackEvent = async (eventData) => {
        if (!config.measurementId) {
            console.warn('Avila Analytics: No measurement ID provided');
            return;
        }

        const envelope = {
            measurement_id: config.measurementId,
            timestamp: new Date().toISOString(),
            event: eventData,
            processed: false,
            event_id: crypto.randomUUID(),
        };

        const params = {
            client_id: getClientId(),
            session_id: getSessionId(),
            ...getDeviceInfo(),
        };

        // Merge params into event
        if (envelope.event.params) {
            envelope.event.params = { ...envelope.event.params, ...params };
        } else {
            envelope.event.params = params;
        }

        if (config.debug) {
            console.log('Tracking event:', envelope);
        }

        try {
            const response = await fetch(`${config.endpoint}/api/v1/collect`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(envelope),
                keepalive: true,
            });

            if (!response.ok && config.debug) {
                console.warn('Avila Analytics: Failed to send event', response.status);
            }
        } catch (err) {
            if (config.debug) {
                console.error('Avila Analytics: Error sending event', err);
            }
        }
    };

    // Track page view
    const trackPageView = () => {
        trackEvent({
            event_type: 'page_view',
            page_title: document.title,
            page_location: window.location.href,
            page_referrer: document.referrer || null,
            user_id: null,
            params: {},
        });
    };

    // Track click
    const trackClick = (element) => {
        trackEvent({
            event_type: 'click',
            element_id: element.id || null,
            element_class: element.className || null,
            element_text: element.textContent?.substr(0, 100) || null,
            link_url: element.href || null,
            params: {},
        });
    };

    // Track scroll
    let scrollTracked = { 25: false, 50: false, 75: false, 90: false };
    const trackScroll = () => {
        const scrollPercent = Math.round(
            (window.scrollY / (document.documentElement.scrollHeight - window.innerHeight)) * 100
        );

        for (const threshold of [25, 50, 75, 90]) {
            if (scrollPercent >= threshold && !scrollTracked[threshold]) {
                scrollTracked[threshold] = true;
                trackEvent({
                    event_type: 'scroll',
                    percent_scrolled: threshold,
                    params: {},
                });
            }
        }
    };

    // Track form submit
    const trackFormSubmit = (form) => {
        trackEvent({
            event_type: 'form_submit',
            form_id: form.id || 'unknown',
            form_name: form.name || null,
            params: {},
        });
    };

    // Track file download
    const trackFileDownload = (link) => {
        const url = new URL(link.href);
        const filename = url.pathname.split('/').pop();
        const extension = filename.split('.').pop();

        trackEvent({
            event_type: 'file_download',
            file_name: filename,
            file_extension: extension,
            link_url: link.href,
            params: {},
        });
    };

    // Auto-tracking setup
    if (config.autoTrack) {
        // Track initial page view
        if (document.readyState === 'complete') {
            trackPageView();
        } else {
            window.addEventListener('load', trackPageView);
        }

        // Track clicks on links
        document.addEventListener('click', (e) => {
            const link = e.target.closest('a');
            if (link) {
                // Check if it's a download
                if (link.download || /\.(pdf|zip|doc|xls|ppt)$/i.test(link.href)) {
                    trackFileDownload(link);
                } else {
                    trackClick(link);
                }
            }
        });

        // Track form submissions
        document.addEventListener('submit', (e) => {
            if (e.target.tagName === 'FORM') {
                trackFormSubmit(e.target);
            }
        });

        // Track scroll depth
        let scrollTimeout;
        window.addEventListener('scroll', () => {
            clearTimeout(scrollTimeout);
            scrollTimeout = setTimeout(trackScroll, 100);
        });
    }

    // Public API
    window.avila = {
        track: (eventName, params = {}) => {
            trackEvent({
                event_type: 'custom',
                name: eventName,
                params,
            });
        },

        trackPageView,

        trackEcommerce: (type, data) => {
            trackEvent({
                event_type: type,
                ...data,
                params: {},
            });
        },

        setUserId: (userId) => {
            localStorage.setItem('avila_user_id', userId);
        },

        getUserId: () => {
            return localStorage.getItem('avila_user_id');
        },
    };

    if (config.debug) {
        console.log('Avila Analytics initialized', config);
    }
})();
