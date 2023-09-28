var { 
    OverlayScrollbars, 
    ScrollbarsHidingPlugin, 
    SizeObserverPlugin, 
    ClickScrollPlugin  
  } = OverlayScrollbarsGlobal;

export function addScroll(){
    OverlayScrollbars(document.getElementById("wrapper"),{
        showNativeOverlaidScrollbars: false,
        overflow:{
            x: 'hidden',
            y: 'scroll',
        },
        scrollbars: {
            theme: 'os-theme-dark',
            visibility: 'visible',
        }
    })
}