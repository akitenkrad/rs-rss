import mermaid from 'mermaid';
import { useEffect, useRef, useState } from 'react';

const MermaidRenderer = ({ chart, id }) => {
    const ref = useRef(null);
    const [error, setError] = useState(null);

    useEffect(() => {
        // Mermaidの初期化
        mermaid.initialize({
            startOnLoad: false,
            theme: 'default',
            securityLevel: 'loose',
            fontFamily: 'monospace',
            themeVariables: {
                primaryColor: '#4299e1',
                primaryTextColor: '#1a202c',
                primaryBorderColor: '#2d3748',
                lineColor: '#4a5568',
                secondaryColor: '#e2e8f0',
                tertiaryColor: '#f7fafc'
            }
        });

        const renderChart = async () => {
            if (ref.current && chart && chart.trim()) {
                try {
                    setError(null);
                    // 既存の内容をクリア
                    ref.current.innerHTML = '';
                    
                    // ユニークなIDを生成
                    const uniqueId = `mermaid-${id}-${Date.now()}`;
                    
                    // チャートの検証とレンダリング
                    const { svg } = await mermaid.render(uniqueId, chart.trim());
                    
                    if (ref.current) {
                        ref.current.innerHTML = svg;
                    }
                } catch (error) {
                    console.error('Mermaid rendering error:', error);
                    setError(error.message || 'チャートをレンダリングできませんでした');
                    if (ref.current) {
                        ref.current.innerHTML = `
                            <div class="mermaid-error">
                                <strong>Mermaid構文エラー:</strong><br/>
                                ${error.message || 'チャートをレンダリングできませんでした'}
                            </div>
                        `;
                    }
                }
            }
        };

        renderChart();
    }, [chart, id]);

    return (
        <div className="mermaid-diagram">
            <div 
                ref={ref} 
                className="mermaid-content"
            />
        </div>
    );
};

export default MermaidRenderer;
