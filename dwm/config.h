/* appearance */
static const unsigned int borderpx  = 1;        /* border pixel of windows */
static const int gappx              = 10;
static const unsigned int snap      = 32;       /* snap pixel */
static const int showbar            = 1;        /* 0 means no bar */
static const int topbar             = 1;        /* 0 means bottom bar */
static const char *fonts[]          = { "FuraCode Nerd Font:size=16" };
static const char dmenufont[]       = "FuraCode Nerd Font:size=16";
static char normbgcolor[]           = "#222222";
static char normbordercolor[]       = "#444444";
static char normfgcolor[]           = "#bbbbbb";
static char selfgcolor[]            = "#eeeeee";
static char selbordercolor[]        = "#005577";
static char selbgcolor[]            = "#005577";

char *colors[][3]      = {
	/*               fg                  bg         border   */
	[SchemeNorm] = { normfgcolor, normbgcolor, normbordercolor },
	[SchemeSel]  = { selfgcolor,  selbgcolor,  selbordercolor  },
  [SchemeHid]  = { selbgcolor,  selfgcolor,  normbordercolor },
};

/* tagging */
static const char *tags[] = { "1", "2" };

static const Rule rules[] = {
	/* xprop(1):
	 *	WM_CLASS(STRING) = instance, class
	 *	WM_NAME(STRING) = title
	 */
	/* class      instance    title       tags mask     isfloating   monitor */
	{ "Gimp",     NULL,       NULL,       0,            1,           -1 },
	{ "Firefox",  NULL,       NULL,       1 << 8,       0,           -1 },
};

/* layout(s) */
static const float mfact     = 0.55; /* factor of master area size [0.05..0.95] */
static const int nmaster     = 1;    /* number of clients in master area */
static const int resizehints = 0;    /* 1 means respect size hints in tiled resizals */

static const Layout layouts[] = {
	/* symbol     arrange function */
	{ "[]=",      tile },    /* first entry is default */
	{ "[M]",      monocle },
};

/* key definitions */
#define MODKEY Mod4Mask
#define XF86AudioMute		      0x1008ff12
#define XF86AudioLowerVolume	0x1008ff11
#define XF86AudioRaiseVolume	0x1008ff13
#define DecreaseBrightness    0x1008ff03
#define IncreaseBrightness    0x1008ff02

/* commands */
static char dmenumon[2] = "0"; /* component of dmenucmd, manipulated in spawn() */
static const char *dmenucmd[] = { "dmenu_run", "-b", "-m", dmenumon, "-fn", dmenufont, "-nb", normbgcolor, "-nf", normfgcolor, "-sb", selbgcolor, "-sf", selfgcolor, NULL };
static const char *termcmd[]  = { "kitty", NULL };
static const char *browsercmd[] = { "firefox", NULL };
static const char *bitwardencmd[] = { "bitwarden", NULL };
static const char *editorcmd[] = { "emacs", NULL };
static const char *themecmd[] = { "ith", NULL };
static const char *logmeoutcmd[] = { "logmeout", NULL };

static const char *volmutecmd[] = { "volume", "--block", "02-volume", "--command", "mute", NULL };
static const char *volinccmd[] = { "volume", "--block", "02-volume", "--command", "increase", NULL };
static const char *voldeccmd[] = { "volume", "--block", "02-volume", "--command", "decrease", NULL };
static const char *scrinccmd[] = { "brightness", "--block", "00-brightness", "--command", "increase", NULL };
static const char *scrdeccmd[] = { "brightness", "--block", "00-brightness", "--command", "decrease", NULL };
static const char *wallpapercmd[] = { "wallpaper", NULL };

static Key keys[] = {
	/* modifier                     key                   function        argument */
	{ MODKEY,                       XK_p,                 spawn,          {.v = dmenucmd } },
	{ MODKEY,                       XK_Return,            spawn,          {.v = termcmd } },
	{ MODKEY,                       XK_b,                 spawn,          {.v = bitwardencmd } },
	{ MODKEY,                       XK_g,                 spawn,          {.v = browsercmd } },
	{ MODKEY,                       XK_e,                 spawn,          {.v = editorcmd } },
	{ MODKEY,                       XK_i,                 spawn,          {.v = themecmd } },
	{ MODKEY,                       XK_n,                 spawn,          {.v = wallpapercmd } },
	{ MODKEY|ShiftMask,             XK_q,                 spawn,          {.v = logmeoutcmd } },

	{ False,                        IncreaseBrightness,   spawn,          {.v = scrinccmd } },
	{ False,                        DecreaseBrightness,   spawn,          {.v = scrdeccmd } },
	{ False,                        XF86AudioRaiseVolume, spawn,          {.v = volinccmd } },
	{ False,                        XF86AudioLowerVolume, spawn,          {.v = voldeccmd } },
	{ False,                        XF86AudioMute,        spawn,          {.v = volmutecmd } },

	{ MODKEY|ShiftMask,             XK_b,                 togglebar,      {0} },

	{ MODKEY,                       XK_h,                 setmfact,       {.f = -0.05} },
	{ MODKEY,                       XK_j,                 focusstack,     {.i = +1 } },
	{ MODKEY,                       XK_k,                 focusstack,     {.i = -1 } },
	{ MODKEY,                       XK_l,                 setmfact,       {.f = +0.05} },

	{ MODKEY,                       XK_equal,             setgaps,        {.i = -10 } },
	{ MODKEY,                       XK_0,                 setgaps,        {.i = 0 } },
	{ MODKEY,                       XK_minus,             setgaps,        {.i = +10 } },

	{ MODKEY|ShiftMask,             XK_h,                 incnmaster,     {.i = +1 } },
	{ MODKEY|ShiftMask,             XK_j,                 movestack,      {.i = +1 } },
	{ MODKEY|ShiftMask,             XK_k,                 movestack,      {.i = -1 } },
	{ MODKEY|ShiftMask,             XK_l,                 incnmaster,     {.i = -1 } },

	{ MODKEY,                       XK_space,             zoom,           {0} },
	{ Mod1Mask,                     XK_Tab,               focusstack,     {.i = + 1} },
	{ Mod1Mask|ShiftMask,           XK_Tab,               focusstack,     {.i = + 1} },
	{ MODKEY,                       XK_d,                 killclient,     {0} },
	{ MODKEY,                       XK_f,                 setlayout,      {0} },
	{ MODKEY|ShiftMask,             XK_f,                 togglefloating, {0} },
	{ MODKEY,                       XK_0,                 view,           {.ui = ~0 } },
	{ MODKEY,                       XK_u,                 focusmon,       {.i = +1 } },
	{ MODKEY,                       XK_y,                 shiftview,      {.i = +1 } },
	{ MODKEY|ShiftMask,             XK_u,                 tagmon,         {.i = +1 } },
	{ MODKEY|ShiftMask,             XK_y,                 shifttag,       {.i = +1 } },
};

/* button definitions */
/* click can be ClkTagBar, ClkLtSymbol, ClkStatusText, ClkWinTitle, ClkClientWin, or ClkRootWin */
static Button buttons[] = {
	/* click                event mask      button          function        argument */
	{ ClkLtSymbol,          0,              Button1,        setlayout,      {0} },
	{ ClkWinTitle,          0,              Button1,        togglewin,      {0} },
	{ ClkWinTitle,          0,              Button2,        zoom,           {0} },
	{ ClkStatusText,        0,              Button2,        spawn,          {.v = termcmd } },
	{ ClkClientWin,         MODKEY,         Button1,        movemouse,      {0} },
	{ ClkClientWin,         MODKEY,         Button2,        togglefloating, {0} },
	{ ClkClientWin,         MODKEY,         Button3,        resizemouse,    {0} },
	{ ClkTagBar,            0,              Button1,        view,           {0} },
	{ ClkTagBar,            0,              Button3,        toggleview,     {0} },
	{ ClkTagBar,            MODKEY,         Button1,        tag,            {0} },
	{ ClkTagBar,            MODKEY,         Button3,        toggletag,      {0} },
};

static Signal signals[] = {
	/* signum       function        argument  */
	{ 1,            xrdb,      {.v = NULL} },
};
