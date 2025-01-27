<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Your session is about to expire. Click_7c440a</name>
   <tag></tag>
   <elementGuidId>8e6144f4-25cb-4980-b067-aa1df121eb8b</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body.slBg</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>3a5d9db0-e1c4-43be-8635-e4c253ca176f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>slBg</value>
      <webElementGuid>0919a89b-ea4b-4302-90a9-f51dba61405a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    

    
    
    
    
    




    








    



    
        
            Your session is about to expire. Click Continue to extend your session.
            
                Continue
            
        
    








    
        
        SiteLink myHub
    
    Smart Management Software for Self-Storage
    Visit the SiteLink myHub information page  to learn more. For a Demo login, contact Sales.
    
    

        
            

    
        
            SiteLink Web Edition
        
        
            
            
            
                var d = new Date();
                document.getElementById(&quot;Client_GmtOffset&quot;).setAttribute('value', d.getTimezoneOffset());
            

            
                
                    
                
                
                    Corp Code
                    
                        
                        
                    
                
                
                    Location Code
                    
                        
                        
                    
                
                
                    User Name
                    
                        
                        
                    
                
                
                    Password
                    
                        
                        The Password field is required.
                    
                
            
        
        
            
        
    

        

        

            
                
    
        
            Corporate Control Center
        
        
            
            
            
                var d = new Date();
                document.getElementById(&quot;Corp_GmtOffset&quot;).setAttribute('value', -d.getTimezoneOffset() / 60);
            
            
                
                    
                
                
                    Corp Code
                    
                        
                        
                    
                
                
                    User Name
                    
                        
                        
                    
                
                
                    Password
                    
                        
                        
                    
                
                
                     
                    
                         
                    
                
            
        
        
            
        
    

            
    
    
        
            
                
                
                
                
                
            
        
    


    window.onbeforeunload = function()
    {
        ShowLoader();
    }

    $(document).ready(function()
    {
        checkOldCookie();
    });

    function ShowLoader()
    {
        $('#full-overlay').show();

    }

    function checkOldCookie()
    {
        if (!getCookie('HubClientLogin'))
        {
            var oldCookie = getCookie('ClientLogin');
            if (oldCookie)
            {
                $('#Client_CorpCode').val(getKeyValue(oldCookie, 'CorpCode'));
                $('#Client_LocationCode').val(getKeyValue(oldCookie, 'LocationCode'));
                $('#Client_UserName').val(getKeyValue(oldCookie, 'UserName'));
            }

        }

        if (!getCookie('HubCorpLogin'))
        {
            var oldCookie = getCookie('CorpLogin');
            if (oldCookie)
            {
                $('#Corp_CorpCode').val(getKeyValue(oldCookie, 'CorpCode'));
                $('#Corp_UserName').val(getKeyValue(oldCookie, 'UserName'));
            }
        }
    }

    function getCookie(cname)
    {
        var name = cname + &quot;=&quot;;
        var decodedCookie = decodeURIComponent(document.cookie);
        var ca = decodedCookie.split(';');
        for (var i = 0; i &lt; ca.length; i++)
        {
            var c = ca[i];
            while (c.charAt(0) == ' ')
            {
                c = c.substring(1);
            }
            if (c.indexOf(name) == 0)
            {
                return c.substring(name.length, c.length);
            }
        }
        return &quot;&quot;;
    }

    function setCookie(cname, cvalue, days, domain)
    {
        var d = new Date();
        d.setDate(d.getDate() + days);
        var expires = &quot;expires=&quot; + d.toUTCString();
        var cookieString = cname + &quot;=&quot; + cvalue + &quot;;&quot; + &quot;SameSite=None; Secure;&quot; + expires + &quot;;path=/&quot;;
        if (domain)
            cookieString += ';domain=' + domain;
        document.cookie = cookieString;
    }

    function getKeyValue(cookieString, Key)
    {
        var split = cookieString.split('&amp;');
        for (var i = 0; i &lt; split.length ; i++)
        {
            var keyValuePair = split[i].split('=');
            if (keyValuePair.length == 2 &amp;&amp; keyValuePair[0] == Key)
            {
                return keyValuePair[1];
            }
        }
        return &quot;&quot;;
    }


    
        
            
                © 2025 SiteLink Software, LLC.  | 
                Terms and Conditions  | Privacy Policy
            
            
        
    



    


id(&quot;Client_Password&quot;)</value>
      <webElementGuid>b147730a-87d4-475e-8bc1-12ae3488ff0e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;js flexbox flexboxlegacy canvas canvastext webgl no-touch geolocation postmessage no-websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms csstransforms3d csstransitions fontface generatedcontent video audio localstorage sessionstorage webworkers no-applicationcache svg inlinesvg smil svgclippaths&quot;]/body[@class=&quot;slBg&quot;]</value>
      <webElementGuid>681d3c38-fbfa-43c5-9999-ae8d1cc9471b</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>0f140fe7-aba7-4a40-ab55-b4e5dd01b544</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
    

    
    
    
    
    




    








    



    
        
            Your session is about to expire. Click Continue to extend your session.
            
                Continue
            
        
    








    
        
        SiteLink myHub
    
    Smart Management Software for Self-Storage
    Visit the SiteLink myHub information page  to learn more. For a Demo login, contact Sales.
    
    

        
            

    
        
            SiteLink Web Edition
        
        
            
            
            
                var d = new Date();
                document.getElementById(&quot;Client_GmtOffset&quot;).setAttribute(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, d.getTimezoneOffset());
            

            
                
                    
                
                
                    Corp Code
                    
                        
                        
                    
                
                
                    Location Code
                    
                        
                        
                    
                
                
                    User Name
                    
                        
                        
                    
                
                
                    Password
                    
                        
                        The Password field is required.
                    
                
            
        
        
            
        
    

        

        

            
                
    
        
            Corporate Control Center
        
        
            
            
            
                var d = new Date();
                document.getElementById(&quot;Corp_GmtOffset&quot;).setAttribute(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, -d.getTimezoneOffset() / 60);
            
            
                
                    
                
                
                    Corp Code
                    
                        
                        
                    
                
                
                    User Name
                    
                        
                        
                    
                
                
                    Password
                    
                        
                        
                    
                
                
                     
                    
                         
                    
                
            
        
        
            
        
    

            
    
    
        
            
                
                
                
                
                
            
        
    


    window.onbeforeunload = function()
    {
        ShowLoader();
    }

    $(document).ready(function()
    {
        checkOldCookie();
    });

    function ShowLoader()
    {
        $(&quot; , &quot;'&quot; , &quot;#full-overlay&quot; , &quot;'&quot; , &quot;).show();

    }

    function checkOldCookie()
    {
        if (!getCookie(&quot; , &quot;'&quot; , &quot;HubClientLogin&quot; , &quot;'&quot; , &quot;))
        {
            var oldCookie = getCookie(&quot; , &quot;'&quot; , &quot;ClientLogin&quot; , &quot;'&quot; , &quot;);
            if (oldCookie)
            {
                $(&quot; , &quot;'&quot; , &quot;#Client_CorpCode&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;CorpCode&quot; , &quot;'&quot; , &quot;));
                $(&quot; , &quot;'&quot; , &quot;#Client_LocationCode&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;LocationCode&quot; , &quot;'&quot; , &quot;));
                $(&quot; , &quot;'&quot; , &quot;#Client_UserName&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;UserName&quot; , &quot;'&quot; , &quot;));
            }

        }

        if (!getCookie(&quot; , &quot;'&quot; , &quot;HubCorpLogin&quot; , &quot;'&quot; , &quot;))
        {
            var oldCookie = getCookie(&quot; , &quot;'&quot; , &quot;CorpLogin&quot; , &quot;'&quot; , &quot;);
            if (oldCookie)
            {
                $(&quot; , &quot;'&quot; , &quot;#Corp_CorpCode&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;CorpCode&quot; , &quot;'&quot; , &quot;));
                $(&quot; , &quot;'&quot; , &quot;#Corp_UserName&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;UserName&quot; , &quot;'&quot; , &quot;));
            }
        }
    }

    function getCookie(cname)
    {
        var name = cname + &quot;=&quot;;
        var decodedCookie = decodeURIComponent(document.cookie);
        var ca = decodedCookie.split(&quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot;);
        for (var i = 0; i &lt; ca.length; i++)
        {
            var c = ca[i];
            while (c.charAt(0) == &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;)
            {
                c = c.substring(1);
            }
            if (c.indexOf(name) == 0)
            {
                return c.substring(name.length, c.length);
            }
        }
        return &quot;&quot;;
    }

    function setCookie(cname, cvalue, days, domain)
    {
        var d = new Date();
        d.setDate(d.getDate() + days);
        var expires = &quot;expires=&quot; + d.toUTCString();
        var cookieString = cname + &quot;=&quot; + cvalue + &quot;;&quot; + &quot;SameSite=None; Secure;&quot; + expires + &quot;;path=/&quot;;
        if (domain)
            cookieString += &quot; , &quot;'&quot; , &quot;;domain=&quot; , &quot;'&quot; , &quot; + domain;
        document.cookie = cookieString;
    }

    function getKeyValue(cookieString, Key)
    {
        var split = cookieString.split(&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;);
        for (var i = 0; i &lt; split.length ; i++)
        {
            var keyValuePair = split[i].split(&quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;);
            if (keyValuePair.length == 2 &amp;&amp; keyValuePair[0] == Key)
            {
                return keyValuePair[1];
            }
        }
        return &quot;&quot;;
    }


    
        
            
                © 2025 SiteLink Software, LLC.  | 
                Terms and Conditions  | Privacy Policy
            
            
        
    



    


id(&quot;Client_Password&quot;)&quot;) or . = concat(&quot;
    

    
    
    
    
    




    








    



    
        
            Your session is about to expire. Click Continue to extend your session.
            
                Continue
            
        
    








    
        
        SiteLink myHub
    
    Smart Management Software for Self-Storage
    Visit the SiteLink myHub information page  to learn more. For a Demo login, contact Sales.
    
    

        
            

    
        
            SiteLink Web Edition
        
        
            
            
            
                var d = new Date();
                document.getElementById(&quot;Client_GmtOffset&quot;).setAttribute(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, d.getTimezoneOffset());
            

            
                
                    
                
                
                    Corp Code
                    
                        
                        
                    
                
                
                    Location Code
                    
                        
                        
                    
                
                
                    User Name
                    
                        
                        
                    
                
                
                    Password
                    
                        
                        The Password field is required.
                    
                
            
        
        
            
        
    

        

        

            
                
    
        
            Corporate Control Center
        
        
            
            
            
                var d = new Date();
                document.getElementById(&quot;Corp_GmtOffset&quot;).setAttribute(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, -d.getTimezoneOffset() / 60);
            
            
                
                    
                
                
                    Corp Code
                    
                        
                        
                    
                
                
                    User Name
                    
                        
                        
                    
                
                
                    Password
                    
                        
                        
                    
                
                
                     
                    
                         
                    
                
            
        
        
            
        
    

            
    
    
        
            
                
                
                
                
                
            
        
    


    window.onbeforeunload = function()
    {
        ShowLoader();
    }

    $(document).ready(function()
    {
        checkOldCookie();
    });

    function ShowLoader()
    {
        $(&quot; , &quot;'&quot; , &quot;#full-overlay&quot; , &quot;'&quot; , &quot;).show();

    }

    function checkOldCookie()
    {
        if (!getCookie(&quot; , &quot;'&quot; , &quot;HubClientLogin&quot; , &quot;'&quot; , &quot;))
        {
            var oldCookie = getCookie(&quot; , &quot;'&quot; , &quot;ClientLogin&quot; , &quot;'&quot; , &quot;);
            if (oldCookie)
            {
                $(&quot; , &quot;'&quot; , &quot;#Client_CorpCode&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;CorpCode&quot; , &quot;'&quot; , &quot;));
                $(&quot; , &quot;'&quot; , &quot;#Client_LocationCode&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;LocationCode&quot; , &quot;'&quot; , &quot;));
                $(&quot; , &quot;'&quot; , &quot;#Client_UserName&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;UserName&quot; , &quot;'&quot; , &quot;));
            }

        }

        if (!getCookie(&quot; , &quot;'&quot; , &quot;HubCorpLogin&quot; , &quot;'&quot; , &quot;))
        {
            var oldCookie = getCookie(&quot; , &quot;'&quot; , &quot;CorpLogin&quot; , &quot;'&quot; , &quot;);
            if (oldCookie)
            {
                $(&quot; , &quot;'&quot; , &quot;#Corp_CorpCode&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;CorpCode&quot; , &quot;'&quot; , &quot;));
                $(&quot; , &quot;'&quot; , &quot;#Corp_UserName&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;UserName&quot; , &quot;'&quot; , &quot;));
            }
        }
    }

    function getCookie(cname)
    {
        var name = cname + &quot;=&quot;;
        var decodedCookie = decodeURIComponent(document.cookie);
        var ca = decodedCookie.split(&quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot;);
        for (var i = 0; i &lt; ca.length; i++)
        {
            var c = ca[i];
            while (c.charAt(0) == &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;)
            {
                c = c.substring(1);
            }
            if (c.indexOf(name) == 0)
            {
                return c.substring(name.length, c.length);
            }
        }
        return &quot;&quot;;
    }

    function setCookie(cname, cvalue, days, domain)
    {
        var d = new Date();
        d.setDate(d.getDate() + days);
        var expires = &quot;expires=&quot; + d.toUTCString();
        var cookieString = cname + &quot;=&quot; + cvalue + &quot;;&quot; + &quot;SameSite=None; Secure;&quot; + expires + &quot;;path=/&quot;;
        if (domain)
            cookieString += &quot; , &quot;'&quot; , &quot;;domain=&quot; , &quot;'&quot; , &quot; + domain;
        document.cookie = cookieString;
    }

    function getKeyValue(cookieString, Key)
    {
        var split = cookieString.split(&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;);
        for (var i = 0; i &lt; split.length ; i++)
        {
            var keyValuePair = split[i].split(&quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;);
            if (keyValuePair.length == 2 &amp;&amp; keyValuePair[0] == Key)
            {
                return keyValuePair[1];
            }
        }
        return &quot;&quot;;
    }


    
        
            
                © 2025 SiteLink Software, LLC.  | 
                Terms and Conditions  | Privacy Policy
            
            
        
    



    


id(&quot;Client_Password&quot;)&quot;))]</value>
      <webElementGuid>3920be88-f783-4eac-b6aa-6320b603fad0</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
