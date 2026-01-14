import * as React from "react"

const MOBILE_BREAKPOINT = 768

export function useIsMobile() {
  const [isMobile, setIsMobile] = React.useState<boolean | undefined>(undefined)

  React.useEffect(() => {
    const mql = window.matchMedia(`(max-width: ${MOBILE_BREAKPOINT - 1}px)`)
    
    // Enhanced mobile detection to support landscape modes on mobile devices
    const checkMobile = () => {
      const userAgent = typeof navigator === 'undefined' ? '' : navigator.userAgent;
      const isMobileDevice = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(userAgent) || 
                            (userAgent.includes("Mac") && "ontouchend" in document); // iPadOS 13+ detection
      
      return window.innerWidth < MOBILE_BREAKPOINT || isMobileDevice;
    }

    const onChange = () => {
      setIsMobile(checkMobile())
    }
    
    mql.addEventListener("change", onChange)
    setIsMobile(checkMobile())
    
    return () => mql.removeEventListener("change", onChange)
  }, [])

  return !!isMobile
}
