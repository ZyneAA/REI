import { useEffect, useState } from "react";
import { Cloud, Sun, CloudRain, Wind } from "lucide-react";
import { ScaleLoader } from "react-spinners";

export default function ElectraHeart() {
    const [weather, setWeather] = useState(null);
    const [forecast, setForecast] = useState(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [city, setCity] = useState("mandalay"); // default city
    const [go, setGo] = useState(false);

    useEffect(() => {
        if (!go) return; // only fetch after Get Started is clicked
        const fetchWeather = async () => {
            setLoading(true);
            setError(null);
            try {
                const res = await fetch(`http://127.0.0.1:8080/${city}`);
                if (!res.ok) throw new Error("Failed to fetch weather");
                const data = await res.json();
                setWeather({
                    city: data.location.name,
                    country: data.location.country,
                    temp: data.current.temp_c,
                    windchill: data.current.windchill_c || "--",
                    heatindex: data.current.heatindex_c || "--",
                    visibility: data.current.vis_km,
                    wind: `${data.current.wind_kph} kph ${data.current.wind_dir}`,
                    humidity: data.current.humidity,
                    uv: data.current.uv,
                    pressure: `${data.current.pressure_mb} mb`,
                    cloud: `${data.current.cloud}%`,
                });

                setForecast(
                    data.forecast.forecastday.map((day) => ({
                        date: day.date,
                        temp: `${day.day.avgtemp_c}°C`,
                        precip: `${day.day.totalprecip_mm} mm`,
                        uv: day.day.uv,
                    }))
                );
            } catch (err) {
                setError(err.message);
            } finally {
                setLoading(false);
            }
        };
        fetchWeather();
    }, [city, go]);

    function FeatureCard({ icon, title, desc }) {
        return (
            <div className="bg-white/20 backdrop-blur-lg rounded-2xl p-6 shadow-lg text-center hover:scale-105 transition">
                <div className="flex justify-center mb-4">{icon}</div>
                <h2 className="text-xl font-bold mb-2">{title}</h2>
                <p className="text-white/80">{desc}</p>
            </div>
        );
    }

    const Card = ({ title, value, big }) => (
        <div
            className={`bg-white/20 backdrop-blur-lg rounded-2xl p-4 text-center shadow-lg ${
                big ? "col-span-4 row-span-2 flex flex-col justify-center" : ""
            }`}
        >
            <h2 className="text-lg font-semibold mb-2">{title}</h2>
            <p className={`font-bold ${big ? "text-6xl" : "text-2xl"}`}>{value}</p>
        </div>
    );

    // Landing Page
    if (!go) {
        return (
            <div className="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-sky-500 via-blue-600 to-indigo-700 text-white p-6">
                <div className="text-center max-w-3xl">
                    <div className="flex justify-center space-x-4 mb-6">
                        <Sun className="w-16 h-16 text-yellow-300 animate-pulse" />
                        <Cloud className="w-16 h-16 text-gray-200" />
                        <CloudRain className="w-16 h-16 text-blue-300" />
                        <Wind className="w-16 h-16 text-teal-200" />
                    </div>

                    <h1 className="text-5xl font-extrabold mb-4 drop-shadow-lg">
                        REI Weather
                    </h1>
                    <p className="text-lg text-white/80 mb-8">
                        Stay ahead of the skies 🌤️ — real-time weather updates and forecasts
                        for your favorite cities.
                    </p>

                    <button
                        onClick={() => setGo(true)} // ✅ fixed here
                        className="bg-white text-blue-700 px-6 py-3 rounded-2xl font-semibold text-lg shadow-xl hover:bg-gray-100 transition"
                    >
                        Get Started
                    </button>
                </div>

                <div className="mt-20 grid grid-cols-1 sm:grid-cols-3 gap-8 max-w-5xl">
                    <FeatureCard
                        icon={<Sun className="w-10 h-10 text-yellow-300" />}
                        title="Real-time Data"
                        desc="Get instant weather updates fetched directly from live APIs."
                    />
                    <FeatureCard
                        icon={<Cloud className="w-10 h-10 text-gray-200" />}
                        title="Multi-City Support"
                        desc="Switch between Mandalay, Rangoon, and Tokyo seamlessly."
                    />
                    <FeatureCard
                        icon={<Wind className="w-10 h-10 text-teal-200" />}
                        title="Clean UI"
                        desc="Minimalistic and responsive design for smooth experience."
                    />
                </div>
            </div>
        );
    }

    // Weather App
    if (loading) return <div className="flex justify-center items-center h-screen"><ScaleLoader color="gray" /></div>;
    if (error) return <div className="text-red-500 text-center mt-10">{error}</div>;

    return (
        <div className="min-h-screen bg-gradient-to-br from-blue-500 to-indigo-700 text-white p-6">
            <div className="flex justify-center mb-6">
                <select
                    value={city}
                    onChange={(e) => setCity(e.target.value)}
                    className="bg-blue-600/60 backdrop-blur-lg text-white p-2 rounded-xl shadow-lg focus:outline-none"
                >
                    <option value="mandalay">Mandalay</option>
                    <option value="rangoon">Rangoon</option>
                    <option value="tokyo">Tokyo</option>
                </select>
            </div>

            <h1 className="text-3xl font-bold text-center mb-8">
                {weather.city}, {weather.country}
            </h1>

            <div className="grid grid-cols-4 gap-4 max-w-6xl mx-auto mb-12">
                <Card title="Temperature" value={`${weather.temp}°C`} big />
                <Card title="Wind Chill" value={`${weather.windchill}°C`} />
                <Card title="Heat Index" value={`${weather.heatindex}°C`} />
                <Card title="Visibility" value={`${weather.visibility} km`} />
                <Card title="Wind" value={weather.wind} />
                <Card title="Humidity" value={`${weather.humidity}%`} />
                <Card title="UV Index" value={weather.uv} />
                <Card title="Pressure" value={weather.pressure} />
                <Card title="Cloud Coverage" value={weather.cloud} />
            </div>

            <div className="bg-white/20 backdrop-blur-lg rounded-2xl p-6 shadow-lg max-w-4xl mx-auto">
                <h2 className="text-2xl font-bold mb-4 text-center">3-Day Forecast</h2>
                <table className="w-full text-center">
                    <thead>
                        <tr className="text-lg font-semibold">
                            <th className="p-3">Date</th>
                            <th className="p-3">Temperature</th>
                            <th className="p-3">Precipitation</th>
                            <th className="p-3">UV Index</th>
                        </tr>
                    </thead>
                    <tbody>
                        {forecast.map((day, idx) => (
                            <tr key={idx} className="hover:bg-white/10 transition">
                                <td className="p-3 font-medium">{day.date}</td>
                                <td className="p-3">{day.temp}</td>
                                <td className="p-3">{day.precip}</td>
                                <td className="p-3">{day.uv}</td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    );
}
